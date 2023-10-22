#![allow(dead_code)]

use tracing::info;
use tracing_subscriber;

mod config_manager;
mod definitions;
mod router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting...");

    // let mut config_manager = config_manager::new_config_manager().unwrap();

    let app = router::get_router();

    info!("Server started successfully");
    axum::Server::bind(&"[::]:8080".parse().unwrap())
        .serve(app.await.into_make_service())
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
