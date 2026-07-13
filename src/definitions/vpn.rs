use super::config::Config;
use super::object::Address;
use crate::validation;
use garde::Validate;
use serde::{Deserialize, Serialize};
use structdb_macros::StructDb;

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Default, Debug)]
#[garde(context(Config))]
pub struct VPN {
    #[delegate]
    #[garde(dive)]
    pub wireguard: Wireguard,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Default, Debug)]
#[garde(context(Config))]
#[garde(custom(validation::validate_wireguard_ports))]
pub struct Wireguard {
    #[collection]
    #[garde(dive)]
    pub interfaces: Vec<WireguardInterface>,
    #[collection]
    #[garde(dive)]
    pub peers: Vec<WireguardPeer>,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
#[structdb(key = "name")]
pub struct WireguardInterface {
    #[garde(custom(validation::validate_name))]
    pub name: String,
    pub public_key: String,
    pub private_key: String,
    pub listen_port: u64,
    #[requires(WireguardPeer)]
    pub peers: Vec<String>,
    pub comment: String,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
#[structdb(key = "name")]
pub struct WireguardPeer {
    #[garde(custom(validation::validate_name))]
    pub name: String,
    pub public_key: String,
    pub preshared_key: Option<String>,
    #[requires(Address)]
    pub allowed_ips: Vec<String>,
    pub endpoint: WireguardEndpoint,
    pub persistent_keepalive: Option<u64>,
    pub comment: String,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub enum WireguardEndpoint {
    #[default]
    None,
    Specify {
        #[requires(Address)]
        #[garde(custom(validation::validate_endpoint_host))]
        address: String,
        port: u64,
    },
}
