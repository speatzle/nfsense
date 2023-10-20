#![allow(dead_code)]

mod definitions;

use definitions::Config;

fn main() {
    println!("Hello, world!");

    let mut config = Config::default();
    config.config_version = 1;

    config.network.interfaces.insert(
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

    config.network.interfaces.insert(
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

    config
        .network
        .static_routes
        .push(definitions::network::StaticRoute {
            name: "test1".to_string(),
            interface: "eth0".to_string(),
            gateway: "192.168.1.1".parse().unwrap(),
            destination: "10.42.42.0/24".parse().unwrap(),
            metric: 0,
        });

    let serialized = serde_json::to_string_pretty(&config).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Config = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
