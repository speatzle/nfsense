use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::IpAddr};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Default, Debug)]
pub struct Network {
    pub interfaces: HashMap<String, NetworkInterface>,
    pub static_routes: Vec<StaticRoute>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct NetworkInterface {
    pub alias: String,
    pub comment: String,
    pub interface_type: NetworkInterfaceType,
    pub addressing_mode: AddressingMode,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum NetworkInterfaceType {
    Hardware { device: String },
    Vlan { id: i32, parent: String },
    Bond { members: Vec<String> },
    Bridge { members: Vec<String> },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AddressingMode {
    None,
    Static { address: String },
    DHCP,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct StaticRoute {
    pub name: String,
    pub interface: String,
    pub gateway: IpAddr,
    pub destination: IpNet,
    pub metric: u64,
}
