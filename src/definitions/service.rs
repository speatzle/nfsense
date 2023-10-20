use core::time;
use macaddr::MacAddr8;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Default, Debug)]
pub struct Service {
    pub dhcp_servers: Vec<DHCPServer>,
    pub dns_servers: Vec<DNSServer>,
    pub ntp_servers: Vec<NTPServer>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct DHCPServer {
    pub interface: String,
    pub pool: Vec<String>,
    pub lease_time: time::Duration,
    pub gateway_mode: GatewayMode,
    pub dns_server_mode: DNSServerMode,
    pub ntp_server_mode: NTPServerMode,
    pub reservations: Vec<Reservation>,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct DNSServer {
    pub interface: String,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct NTPServer {
    pub interface: String,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GatewayMode {
    None,
    Interface,
    Specify { gateway: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DNSServerMode {
    None,
    Interface,
    Specify { dns_servers: Vec<String> },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum NTPServerMode {
    None,
    Interface,
    Specify { ntp_servers: Vec<String> },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reservation {
    pub ip_address: IpAddr,
    pub hardware_address: MacAddr8,
    pub comment: String,
}
