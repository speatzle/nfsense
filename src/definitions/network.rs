use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use validator::Validate;

use crate::definitions::Referenceable;
use crate::{impl_referenceable_trait, impl_references_trait};

use super::config::Config;
use super::object::AddressReference;
use super::References;

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct Network {
    pub interfaces: NetworkInterfaces,
    pub static_routes: Vec<StaticRoute>,
}

type NetworkInterfaces = Vec<NetworkInterface>;
impl_referenceable_trait!(NetworkInterfaces, NetworkInterface);

pub type NetworkInterfaceReference = String;
impl_references_trait!(
    NetworkInterfaceReference,
    NetworkInterface,
    network.interfaces
);

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
    Hardware {
        device: String,
    },
    Vlan {
        id: i32,
        parent: NetworkInterfaceReference,
    },
    Bond {
        members: Vec<NetworkInterfaceReference>,
    },
    Bridge {
        members: Vec<NetworkInterfaceReference>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AddressingMode {
    None,
    Static { address: AddressReference },
    DHCP,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct StaticRoute {
    pub name: String,
    pub interface: NetworkInterfaceReference,
    // TODO make this a Address Object Reference?
    pub gateway: IpAddr,
    pub destination: IpNet,
    pub metric: u64,
    pub comment: String,
}
