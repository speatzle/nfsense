use garde::Validate;
use serde::{Deserialize, Serialize};
use structdb_macros::StructDb;

use super::firewall;

use super::network;

use super::object;

use super::service;

use super::system;
use super::vpn;

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Default, Debug)]
#[garde(context(Config))]
pub struct Config {
    #[garde(skip)]
    pub config_version: u64,
    #[delegate]
    #[garde(dive)]
    pub network: network::Network,
    #[delegate]
    #[garde(dive)]
    pub object: object::Object,
    #[delegate]
    #[garde(dive)]
    pub system: system::System,
    #[delegate]
    #[garde(dive)]
    pub service: service::Service,
    #[delegate]
    #[garde(dive)]
    pub vpn: vpn::VPN,
    #[delegate]
    #[garde(dive)]
    pub firewall: firewall::Firewall,
}
