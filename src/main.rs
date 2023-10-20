#![allow(dead_code)]

use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::IpAddr};

#[derive(Serialize, Deserialize, Default, Debug)]
struct Config {
    config_version: u64,
    network: Network,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct Network {
    interfaces: HashMap<String, NetworkInterface>,
    static_routes: Vec<StaticRoute>,
}

#[derive(Serialize, Deserialize, Debug)]
struct NetworkInterface {
    alias: String,
    comment: String,
    interface_type: NetworkInterfaceType,
    addressing_mode: AddressingMode,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum NetworkInterfaceType {
    Hardware { device: String },
    Vlan { id: i32, parent: String },
    Bond { members: Vec<String> },
    Bridge { members: Vec<String> },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum AddressingMode {
    None,
    Static { address: String },
    DHCP,
}

#[derive(Serialize, Deserialize, Debug)]
struct StaticRoute {
    name: String,
    interface: String,
    gateway: IpAddr,
    destination: IpNet,
    metric: u64,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct Object {
    addresses: HashMap<String, Address>,
    services: HashMap<String, Service>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    address_type: AddressType,
    comment: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum AddressType {
    Host { host: String },
    Range { range: IpAddr },
    Network { network: IpNet },
    Group { children: Vec<String> },
}

#[derive(Serialize, Deserialize, Debug)]
struct Service {
    service_type: ServiceType,
    comment: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum ServiceType {
    TCP {
        source_port_start: u64,
        source_port_end: u64,
        destination_port_start: u64,
        destination_port_end: u64,
    },
    UDP {
        range: IpAddr,
    },
    ICMP {
        code: u8,
    },
    Group {
        children: Vec<String>,
    },
}

fn main() {
    println!("Hello, world!");

    let mut config = Config::default();
    config.config_version = 1;

    config.network.interfaces.insert(
        "inter1".to_string(),
        NetworkInterface {
            alias: "test".to_owned(),
            comment: "test comment".to_owned(),
            interface_type: NetworkInterfaceType::Hardware {
                device: "eth0".to_owned(),
            },
            addressing_mode: AddressingMode::None,
        },
    );

    config.network.interfaces.insert(
        "inter2".to_string(),
        NetworkInterface {
            alias: "test2".to_owned(),
            comment: "test comment".to_owned(),
            interface_type: NetworkInterfaceType::Hardware {
                device: "eth0".to_owned(),
            },
            addressing_mode: AddressingMode::Static {
                address: "192.168.1.1".to_owned(),
            },
        },
    );

    config.network.static_routes.push(StaticRoute {
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
