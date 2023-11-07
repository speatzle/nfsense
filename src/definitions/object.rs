use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use validator::Validate;

// Referencing
use crate::definitions::config::Config;
use crate::definitions::Referenceable;
use crate::definitions::References;
use crate::{impl_referenceable_trait, impl_references_trait};

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct Object {
    pub addresses: Addresses,
    pub services: Services,
}

impl_referenceable_trait!(Addresses, Address);
impl_references_trait!(AddressReference, Address, object.addresses);

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct Address {
    pub name: String,
    pub address_type: AddressType,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AddressType {
    Host { address: IpAddr },
    Range { range: IpAddr },
    Network { network: IpNet },
    Group { members: Vec<AddressReference> },
}

impl_referenceable_trait!(Services, Service);
impl_references_trait!(ServiceReference, Service, object.services);

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct Service {
    pub name: String,
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
        members: Vec<ServiceReference>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PortDefinition {
    Any,
    Single { port: u64 },
    Range { start_port: u64, end_port: u64 },
}
