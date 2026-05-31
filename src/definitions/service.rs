use super::config::Config;
use super::network::NetworkInterface;
use super::object::Address;
use garde::Validate;
use macaddr::MacAddr8;
use serde::{Deserialize, Serialize};
use structdb_macros::StructDb;

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Default, Debug)]
#[garde(context(Config))]
pub struct Service {
    #[collection]
    #[garde(dive)]
    pub dhcp_servers: Vec<DHCPServer>,
    #[collection]
    #[garde(dive)]
    pub dns_servers: Vec<DNSServer>,
    #[collection]
    #[garde(dive)]
    pub ntp_servers: Vec<NTPServer>,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug, PartialEq)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct DHCPServer {
    pub name: String,
    #[requires(NetworkInterface)]
    pub interface: String,
    #[requires(Address)]
    pub pool: Vec<String>,
    pub lease_time: u64,
    pub gateway_mode: GatewayMode,
    pub dns_server_mode: DNSServerMode,
    pub ntp_server_mode: NTPServerMode,
    // pub reservations: Vec<Reservation>,
    pub comment: String,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Default, Debug)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct DNSServer {
    pub name: String,
    #[requires(NetworkInterface)]
    pub interface: String,
    pub comment: String,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Default, Debug)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct NTPServer {
    pub name: String,
    #[requires(NetworkInterface)]
    pub interface: String,
    pub comment: String,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug, Default, PartialEq)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct GatewayModeSpecify {
    #[requires(Address)]
    pub gateway: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GatewayMode {
    None,
    Interface,
    Specify(GatewayModeSpecify),
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug, Default, PartialEq)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct DNSServerModeSpecify {
    #[requires(Address)]
    pub dns_servers: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DNSServerMode {
    None,
    Interface,
    Specify(DNSServerModeSpecify),
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug, Default, PartialEq)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct NTPServerModeSpecify {
    #[requires(Address)]
    pub ntp_servers: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NTPServerMode {
    None,
    Interface,
    Specify(NTPServerModeSpecify),
}

#[derive(StructDb, Serialize, Deserialize, Clone, Debug)]
pub struct Reservation {
    pub ip_address: String,
    pub hardware_address: MacAddr8,
    pub comment: String,
}
