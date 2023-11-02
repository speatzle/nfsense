use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::IpAddr};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct Object {
    pub addresses: HashMap<String, Address>,
    pub services: HashMap<String, Service>,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct Address {
    pub address_type: AddressType,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AddressType {
    Host { address: String },
    Range { range: IpAddr },
    Network { network: IpNet },
    Group { members: Vec<String> },
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct Service {
    pub service_type: ServiceType,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ServiceType {
    TCP {
        source: PortDefinition,
        destination: PortDefinition,
    },
    UDP {
        source: PortDefinition,
        destination: PortDefinition,
    },
    ICMP {
        code: u8,
    },
    Group {
        children: Vec<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PortDefinition {
    Any,
    Single { port: u64 },
    Range { start_port: u64, end_port: u64 },
}
