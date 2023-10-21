use serde::{Deserialize, Serialize};
use validator::Validate;

use super::firewall;
use super::network;
use super::object;
use super::service;
use super::system;
use super::vpn;

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct Config {
    pub config_version: u64,
    pub network: network::Network,
    pub object: object::Object,
    pub system: system::System,
    pub service: service::Service,
    pub vpn: vpn::VPN,
    pub firewall: firewall::Firewall,
}
