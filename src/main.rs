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
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use tower_cookies::CookieManagerLayer;
//use tower_http::services::ServeDir;
use tracing::info;
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

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting...");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "generate-default" {
        info!("Generating default config...");
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

    info!("Connecting to System dbus...");
    let dbus_conn = zbus::Connection::system().await.unwrap();

    let app_state = AppState {
        config_manager: config_manager.clone(),
        session_state: session_state.clone(),
        dbus_conn: dbus_conn.clone(),
        rpc_module: api::new_rpc_module(RpcState {
            config_manager,
            session_state,
            dbus_conn,
        }),
    };

    let webinterface_router = ReverseProxy::new("/", "http://localhost:5173");
    /* TODO add flag to server via proxy, default to static files
    let static_files = ServeDir::new("./assets");
    Router::new()
            .route("/", get(|| async {"hello"}))
            .nest_service("/static", static_files)
            */

    // Note: The Router Works Bottom Up, So the auth middleware will only applies to everything above it.
    let main_router = Router::new()
        .merge(web::rpc::routes())
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            web::auth::mw_auth,
        ))
        .merge(web::auth::routes())
        .with_state(app_state)
        .merge(webinterface_router)
        .layer(CookieManagerLayer::new());
    // .fallback_service(service)

    let config =
        OpenSSLConfig::from_pem_file(PathBuf::from("cert.pem"), PathBuf::from("key.pem")).unwrap();

    info!("Server started successfully");
    let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), 8080);
    axum_server::bind_openssl(addr, config)
        .serve(main_router.into_make_service())
        .await
        .unwrap();
}
