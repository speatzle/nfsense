use super::config::Config;
use super::object::{Address, Service};
use crate::validation;
use garde::Validate;
use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use structdb_macros::StructDb;

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Default, Debug)]
#[garde(context(Config))]
pub struct Network {
    #[collection]
    #[garde(dive)]
    pub interfaces: Vec<NetworkInterface>,
    #[collection]
    #[garde(dive)]
    pub static_routes: Vec<StaticRoute>,
    #[collection]
    #[garde(dive)]
    pub policy_routes: Vec<PolicyRoute>,
    #[collection]
    #[garde(dive)]
    pub virtual_routers: Vec<VirtualRouter>,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug, PartialEq)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
#[structdb(key = "name")]
pub struct NetworkInterface {
    #[garde(custom(validation::validate_name))]
    pub name: String,
    pub alias: String,
    pub comment: String,
    pub interface_type: NetworkInterfaceType,
    pub addressing_mode: AddressingMode,
    #[requires(VirtualRouter)]
    pub virtual_router: Option<String>,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug, Default, PartialEq)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct Vlan {
    pub id: i32,
    #[requires(NetworkInterface)]
    pub parent: String,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug, Default, PartialEq)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct Bond {
    #[requires(NetworkInterface)]
    pub members: Vec<String>,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug, Default, PartialEq)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct Bridge {
    #[requires(NetworkInterface)]
    pub members: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NetworkInterfaceType {
    // TODO figure out how to validate the device since it needs to soft fail
    Hardware { device: String },
    Vlan(Vlan),
    Bond(Bond),
    Bridge(Bridge),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AddressingMode {
    None,
    Static {
        address: IpNet,
    },
    #[serde(rename(serialize = "dhcp", deserialize = "dhcp"))]
    DHCP,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug, PartialEq)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
#[structdb(key = "name")]
pub struct StaticRoute {
    #[garde(custom(validation::validate_name))]
    pub name: String,
    #[requires(NetworkInterface)]
    pub interface: String,
    #[requires(Address)]
    pub gateway: String,
    #[requires(Address)]
    pub destination: String,
    pub metric: u64,
    pub comment: String,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug, PartialEq)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
#[structdb(key = "name")]
pub struct PolicyRoute {
    pub name: String,
    #[requires(VirtualRouter)]
    pub source_virtual_router: String,
    #[requires(NetworkInterface)]
    pub source_interfaces: Vec<String>,
    #[requires(Service)]
    pub services: Vec<String>,
    #[requires(Address)]
    pub source_addresses: Vec<String>,
    pub negate_source: bool,
    #[requires(Address)]
    pub destination_addresses: Vec<String>,
    pub negate_destination: bool,
    pub comment: String,
    pub counter: bool,
    pub log: bool,
    pub action: RouteAction,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug, Default, PartialEq)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct RouteActionGateway {
    #[requires(Address)]
    pub address: String,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug, Default, PartialEq)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct RouteActionInterface {
    #[requires(NetworkInterface)]
    pub interface: String,
    #[requires(Address)]
    pub gateway: String,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug, Default, PartialEq)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct RouteActionVirtualRouter {
    #[requires(VirtualRouter)]
    pub virtual_router: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RouteAction {
    Gateway(RouteActionGateway),
    Interface(RouteActionInterface),
    VirtualRouter(RouteActionVirtualRouter),
    Blackhole,
    StopPolicyRouting,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug, PartialEq)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct Link {
    pub name: String,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug, PartialEq)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
#[structdb(key = "name")]
pub struct VirtualRouter {
    #[garde(custom(validation::validate_name))]
    pub name: String,
    // Is limited to u16 even though the kernel supports u32 since conntrack zones are limited to u16
    // TODO limit more for reserved values
    pub table_id: u16,
    pub comment: String,
}
