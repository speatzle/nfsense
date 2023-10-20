pub mod firewall;
pub mod network;
pub mod object;
pub mod service;
pub mod system;
pub mod vpn;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Default, Debug)]
pub struct Config {
    pub config_version: u64,
    pub network: network::Network,
    pub object: object::Object,
    pub system: system::System,
    pub service: service::Service,
    pub vpn: vpn::VPN,
    pub firewall: firewall::Firewall,
}
