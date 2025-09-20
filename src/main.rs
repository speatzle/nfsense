// #![allow(dead_code)]

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::state::RpcState;
use axum::{middleware, Router};
use config_manager::ConfigManager;
use state::AppState;
use std::env;
use std::fs;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
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

    // Note: The Router Works Bottom Up, So the auth middleware will only applies to everything above it.
    let main_router = Router::new()
        .merge(web::rpc::routes())
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            web::auth::mw_auth,
        ))
        .merge(web::auth::routes())
        .with_state(app_state)
        .layer(CookieManagerLayer::new());
    // .fallback_service(service)

    info!("Server started successfully");
    let listener = TcpListener::bind("[::]:8080").await.unwrap();
    axum::serve(listener, main_router).await.unwrap();
}
