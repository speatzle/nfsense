#![allow(dead_code)]

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{middleware, Router};
use config_manager::ConfigManager;
use state::AppState;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber;
use web::auth::SessionState;

mod config_manager;
mod definitions;
mod state;
mod web;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting...");

    // TODO Check Config Manager Setup Error
    let config_manager = ConfigManager::new().unwrap();

    let app_state = AppState {
        config_manager,
        session_state: SessionState {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        },
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
    axum::Server::bind(&"[::]:8080".parse().unwrap())
        .serve(main_router.into_make_service())
        .await
        .unwrap();

    /*
    let mut tx = config_manager.start_transaction().unwrap();

    tx.changes
        .firewall
        .forward_rules
        .push(definitions::firewall::ForwardRule {
            name: "name".to_string(),
            comment: "test".to_string(),
            counter: true,
            verdict: definitions::firewall::Verdict::Accept,
            services: Vec::new(),
            destination_addresses: Vec::new(),
            source_addresses: Vec::new(),
        });

    tx.commit().unwrap();

    config_manager.apply_pending_changes().unwrap();

    let mut tx2 = config_manager.start_transaction().unwrap();

    tx2.changes.network.interfaces.insert(
        "inter1".to_string(),
        definitions::network::NetworkInterface {
            alias: "test".to_owned(),
            comment: "test comment".to_owned(),
            interface_type: definitions::network::NetworkInterfaceType::Hardware {
                device: "eth0".to_owned(),
            },
            addressing_mode: definitions::network::AddressingMode::None,
        },
    );

    tx2.changes.network.interfaces.insert(
        "inter2".to_string(),
        definitions::network::NetworkInterface {
            alias: "test2".to_owned(),
            comment: "test comment".to_owned(),
            interface_type: definitions::network::NetworkInterfaceType::Hardware {
                device: "eth0".to_owned(),
            },
            addressing_mode: definitions::network::AddressingMode::Static {
                address: "192.168.1.1".to_owned(),
            },
        },
    );

    tx2.changes
        .network
        .static_routes
        .push(definitions::network::StaticRoute {
            name: "test1".to_string(),
            interface: "eth0".to_string(),
            gateway: "192.168.1.1".parse().unwrap(),
            destination: "10.42.42.0/24".parse().unwrap(),
            metric: 0,
        });

    tx2.commit().unwrap();

    config_manager.apply_pending_changes().unwrap();

    let applied_config = config_manager.get_current_config();
    info!("applied_config = {:#?}", applied_config);
    */
}
