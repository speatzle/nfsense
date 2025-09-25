use super::config::Config;
use crate::validation;
use garde::Validate;
use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
#[garde(context(Config))]
pub struct Object {
    #[garde(dive)]
    pub addresses: Vec<Address>,
    #[garde(dive)]
    pub services: Vec<Service>,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct Address {
    #[garde(custom(validation::validate_name))]
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
    Group { members: Vec<String> },
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct Service {
    #[garde(custom(validation::validate_name))]
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
        ptypes: Vec<ICMPPacketTypes>,
    },
    Group {
        members: Vec<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PortDefinition {
    Any,
    Single { port: u32 },
    Range { start_port: u32, end_port: u32 },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ICMPPacketTypes {
    EchoReply,
    DestinationUnreachable,
    SourceQuench,
    Redirect,
    EchoRequest,
    TimeExceeded,
    ParameterProblem,
    TimestampRequest,
    TimestampReply,
    InfoRequest,
    InfoReply,
    AddressMaskRequest,
    AddressMaskReply,
    RouterAdvertisement,
    RouterSolicitation,
}
