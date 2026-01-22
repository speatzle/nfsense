use super::config::Config;
use crate::validation;
use garde::Validate;
use ipnet::IpNet;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
#[garde(context(Config))]
pub struct Network {
    #[garde(dive)]
    pub interfaces: Vec<NetworkInterface>,
    #[garde(dive)]
    pub static_routes: Vec<StaticRoute>,
    #[garde(dive)]
    pub policy_routes: Vec<PolicyRoute>,
    #[garde(dive)]
    pub virtual_routers: Vec<VirtualRouter>,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct NetworkInterface {
    #[garde(custom(validation::validate_name))]
    pub name: String,
    pub alias: String,
    pub comment: String,
    pub interface_type: NetworkInterfaceType,
    pub addressing_mode: AddressingMode,
    pub virtual_router: Option<String>,
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
    Static {
        address: IpNet,
    },
    #[serde(rename(serialize = "dhcp", deserialize = "dhcp"))]
    DHCP,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct StaticRoute {
    #[garde(custom(validation::validate_name))]
    pub name: String,
    pub interface: String,
    pub gateway: String,
    pub destination: String,
    pub metric: u64,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct PolicyRoute {
    pub name: String,
    pub source_virtual_router: String,
    pub source_interfaces: Vec<String>,
    pub services: Vec<String>,
    pub source_addresses: Vec<String>,
    pub negate_source: bool,
    pub destination_addresses: Vec<String>,
    pub negate_destination: bool,
    pub comment: String,
    pub counter: bool,
    pub log: bool,
    pub action: RouteAction,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RouteAction {
    Gateway { address: String },
    Interface { interface: String, gateway: String },
    VirtualRouter { virtual_router: String },
    Blackhole,
    StopPolicyRouting,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct Link {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct VirtualRouter {
    #[garde(custom(validation::validate_name))]
    pub name: String,
    // Is limited to u16 even though the kernel supports u32 since conntrack zones are limited to u16
    // TODO limit more for reserved values
    pub table_id: u16,
    pub comment: String,
}
