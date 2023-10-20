use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::IpAddr};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Default, Debug)]
pub struct Object {
    pub addresses: HashMap<String, Address>,
    pub services: HashMap<String, Service>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct Address {
    pub address_type: AddressType,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AddressType {
    Host { host: String },
    Range { range: IpAddr },
    Network { network: IpNet },
    Group { children: Vec<String> },
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct Service {
    pub service_type: ServiceType,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ServiceType {
    TCP {
        source_port: u64,
        source_port_end: Option<u64>,
        destination_port: u64,
        destination_port_end: Option<u64>,
    },
    UDP {
        source_port: u64,
        source_port_end: Option<u64>,
        destination_port: u64,
        destination_port_end: Option<u64>,
    },
    ICMP {
        code: u8,
    },
    Group {
        children: Vec<String>,
    },
}
