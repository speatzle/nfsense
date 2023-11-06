use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::get_thing;

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct VPN {
    pub wireguard: Wireguard,
}

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct Wireguard {
    pub interfaces: Vec<WireguardInterface>,
    pub peers: Vec<WireguardPeer>,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct WireguardInterface {
    pub name: String,
    pub public_key: String,
    pub private_key: String,
    pub listen_port: u64,
    pub peers: Vec<String>,
    pub comment: String,
}

get_thing!(WireguardInterface, get_wireguard_interface);

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct WireguardPeer {
    pub name: String,
    pub public_key: String,
    pub preshared_key: Option<String>,
    pub allowed_ips: Vec<String>,
    pub endpoint: Option<String>,
    pub persistent_keepalive: Option<u64>,
    pub comment: String,
}

get_thing!(WireguardPeer, get_wireguard_peer);
