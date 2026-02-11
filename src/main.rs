// #![allow(dead_code)]

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::state::RpcState;
use axum::{middleware, Router};
use axum_reverse_proxy::ReverseProxy;
use axum_server::tls_openssl::OpenSSLConfig;
use config_manager::ConfigManager;
use state::AppState;
use std::env;
use std::fs;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use tokio::signal;
use tower_cookies::CookieManagerLayer;
use tower_http::services::{ServeDir, ServeFile};
use tracing::{error, info};
use tracing_subscriber;
use web::auth::SessionState;

#[macro_use]
extern crate lazy_static;

mod api;
mod apply;
mod config_manager;
mod definitions;
mod state;
mod templates;
mod validation;
mod web;

pub const CLIENT_INDEX_PATH: &str = "/usr/share/nfsense/html/index.html";
pub const CLIENT_FAVICON_PATH: &str = "/usr/share/nfsense/html/favicon.svg";
pub const CLIENT_ASSETS_PATH: &str = "/usr/share/nfsense/html/assets";
pub const HTTPS_CERT_PATH: &str = "/var/lib/nfsense/cert.pem";
pub const HTTPS_KEY_PATH: &str = "/var/lib/nfsense/key.pem";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting...");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "generate-default" {
        info!("Generating default config...");
        if Path::new(config_manager::CURRENT_CONFIG_PATH).exists() {
            error!("Default config already exists. Not overwriting.");
            return;
        }
        config_manager::generate_default_config(config_manager::CURRENT_CONFIG_PATH).unwrap();
        fs::copy(
            config_manager::CURRENT_CONFIG_PATH,
            config_manager::PENDING_CONFIG_PATH,
        )
        .unwrap();
        info!("Done! Exiting...");
        return;
    }

    // TODO Check Config Manager Setup Error
    let config_manager = ConfigManager::new().unwrap();
    let session_state = SessionState {
        sessions: Arc::new(RwLock::new(HashMap::new())),
    };

    if args.len() > 1 && args[1] == "generate-current-config" {
        match config_manager.clone().generate_current() {
            Ok(_) => info!("Done! Exiting..."),
            Err(_) => {
                error!("Failed to Generate config.");
                std::process::exit(1);
            }
        }
        return;
    }

    info!("Connecting to System dbus...");
    let dbus_conn = zbus::Connection::system().await.unwrap();

    let app_state = AppState {
        session_state: session_state.clone(),
        config_manager: config_manager.clone(),
    };

    let rpc_module = api::new_rpc_module(RpcState {
        config_manager: app_state.config_manager.clone(),
        session_state,
        dbus_conn,
    });

    if args.len() > 1 && args[1] == "apply-pending-config" {
        match config_manager.clone().apply_pending_changes() {
            Ok(_) => info!("Done! Exiting..."),
            Err(err) => error!("Failed to apply config: {}", err),
        }
        return;
    }

    let (rpc_router, rpc_handle) = web::rpc::routes(rpc_module);

    let mut webinterface_router = Router::new()
        .nest_service("/assets", ServeDir::new(CLIENT_ASSETS_PATH))
        .route_service("/favicon.svg", ServeFile::new(CLIENT_FAVICON_PATH))
        .fallback_service(ServeFile::new(CLIENT_INDEX_PATH));

    // Reverse proxy to dev webinterface
    if args.len() > 1 && args[1] == "dev-webinterface" {
        info!("Proxing to dev webinterface");
        webinterface_router =
            Router::new().fallback_service(ReverseProxy::new("/", "http://localhost:5173"));
    }

    // Note: The Router Works Bottom Up, So the auth middleware will only applies to everything above it.
    let main_router = Router::new()
        .merge(rpc_router)
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            web::auth::mw_auth,
        ))
        .merge(web::auth::routes())
        .with_state(app_state)
        .merge(webinterface_router)
        .layer(CookieManagerLayer::new());

    let config = OpenSSLConfig::from_pem_file(
        PathBuf::from(HTTPS_CERT_PATH),
        PathBuf::from(HTTPS_KEY_PATH),
    )
    .unwrap();

    let handle = axum_server::Handle::new();

    let _shutdown_future = shutdown_signal(handle.clone(), rpc_handle);

    info!("Server started successfully");
    let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), 4444);
    axum_server::bind_openssl(addr, config)
        .handle(handle)
        .serve(main_router.into_make_service())
        .await
        .unwrap();
}
// TODO this does not actually handle the signal?
async fn shutdown_signal(handle: axum_server::Handle, rpc_handle: jsonrpsee::server::ServerHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Received termination signal, shutting down");
    match rpc_handle.stop() {
        Ok(_) => info!("RPC server stopped successfully"),
        Err(e) => error!("RPC server stop error: {:?}", e),
    }

    handle.graceful_shutdown(Some(Duration::from_secs(10)));

    tracing::info!("Shutdown Complete");
}
