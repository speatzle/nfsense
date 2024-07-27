use garde::Validate;
use macaddr::MacAddr8;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct Service {
    #[garde(dive)]
    pub dhcp_servers: Vec<DHCPServer>,
    #[garde(dive)]
    pub dns_servers: Vec<DNSServer>,
    #[garde(dive)]
    pub ntp_servers: Vec<NTPServer>,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
#[garde(allow_unvalidated)]
pub struct DHCPServer {
    pub name: String,
    pub interface: String,
    pub pool: Vec<String>,
    pub lease_time: u64,
    pub gateway_mode: GatewayMode,
    pub dns_server_mode: DNSServerMode,
    pub ntp_server_mode: NTPServerMode,
    // pub reservations: Vec<Reservation>,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
#[garde(allow_unvalidated)]
pub struct DNSServer {
    pub name: String,
    pub interface: String,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
#[garde(allow_unvalidated)]
pub struct NTPServer {
    pub name: String,
    pub interface: String,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GatewayMode {
    None,
    Interface,
    Specify { gateway: String },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DNSServerMode {
    None,
    Interface,
    Specify { dns_servers: Vec<String> },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum NTPServerMode {
    None,
    Interface,
    Specify { ntp_servers: Vec<String> },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Reservation {
    pub ip_address: String,
    pub hardware_address: MacAddr8,
    pub comment: String,
}
