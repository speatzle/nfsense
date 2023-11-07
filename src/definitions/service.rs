use core::time;
use macaddr::MacAddr8;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{network::NetworkInterfaceReference, object::AddressReference};

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct Service {
    pub dhcp_servers: Vec<DHCPServer>,
    pub dns_servers: Vec<DNSServer>,
    pub ntp_servers: Vec<NTPServer>,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct DHCPServer {
    pub interface: NetworkInterfaceReference,
    pub pool: Vec<AddressReference>,
    pub lease_time: time::Duration,
    pub gateway_mode: GatewayMode,
    pub dns_server_mode: DNSServerMode,
    pub ntp_server_mode: NTPServerMode,
    pub reservations: Vec<Reservation>,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct DNSServer {
    pub interface: NetworkInterfaceReference,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct NTPServer {
    pub interface: NetworkInterfaceReference,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GatewayMode {
    None,
    Interface,
    Specify { gateway: AddressReference },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DNSServerMode {
    None,
    Interface,
    Specify { dns_servers: Vec<AddressReference> },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum NTPServerMode {
    None,
    Interface,
    Specify { ntp_servers: Vec<AddressReference> },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Reservation {
    pub ip_address: AddressReference,
    pub hardware_address: MacAddr8,
    pub comment: String,
}
