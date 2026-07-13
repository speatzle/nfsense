use super::config::Config;
use crate::validation;
use garde::Validate;
use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use structdb_macros::StructDb;

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Default, Debug)]
#[garde(context(Config))]
pub struct Object {
    #[collection]
    #[garde(dive)]
    pub addresses: Vec<Address>,
    #[collection]
    #[garde(dive)]
    pub services: Vec<Service>,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
#[structdb(key = "name")]
pub struct Address {
    #[garde(custom(validation::validate_name))]
    pub name: String,
    #[delegate]
    pub address_type: AddressType,
    pub comment: String,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug, Default, PartialEq)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct AddressGroup {
    #[requires(Address)]
    pub members: Vec<String>,
}

#[derive(StructDb, Validate, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub enum AddressType {
    Host { address: IpAddr },
    Range { range: IpAddr },
    Network { network: IpNet },
    Group(AddressGroup),
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
#[structdb(key = "name")]
pub struct Service {
    #[garde(custom(validation::validate_name))]
    pub name: String,
    #[delegate]
    pub service_type: ServiceType,
    pub comment: String,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug, Default, PartialEq)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct ServiceGroup {
    #[requires(Service)]
    pub members: Vec<String>,
}

#[derive(StructDb, Validate, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
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
    Group(ServiceGroup),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PortDefinition {
    Any,
    Single { port: u32 },
    Range { start_port: u32, end_port: u32 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
