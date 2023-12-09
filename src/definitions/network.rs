use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use validator::Validate;

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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum NetworkInterfaceType {
    // TODO figure out how to validate the device since it needs to soft fail
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
    // TODO make this a Address Object Reference?
    pub gateway: IpAddr,
    pub destination: IpNet,
    pub metric: u64,
    pub comment: String,
}
