use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use validator::Validate;

use crate::get_thing;

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct Network {
    pub interfaces: Vec<NetworkInterface>,
    pub static_routes: Vec<StaticRoute>,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct NetworkInterface {
    pub name: String,
    pub alias: String,
    pub comment: String,
    pub interface_type: NetworkInterfaceType,
    pub addressing_mode: AddressingMode,
}

get_thing!(NetworkInterface, get_network_interface);

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum NetworkInterfaceType {
    Hardware { device: String },
    Vlan { id: i32, parent: String },
    Bond { members: Vec<String> },
    Bridge { members: Vec<String> },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AddressingMode {
    None,
    Static { address: String },
    DHCP,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct StaticRoute {
    pub name: String,
    pub interface: String,
    pub gateway: IpAddr,
    pub destination: IpNet,
    pub metric: u64,
    pub comment: String,
}
