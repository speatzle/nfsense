use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Default, Debug)]
pub struct VPN {
    pub wireguard: Wireguard,
}

#[derive(Serialize, Deserialize, Validate, Default, Debug)]
pub struct Wireguard {
    pub interfaces: HashMap<String, WireguardInterface>,
    pub peers: HashMap<String, WireguardPeer>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct WireguardInterface {
    pub public_key: String,
    pub private_key: String,
    pub listen_port: u64,
    pub peers: Vec<String>,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct WireguardPeer {
    pub public_key: String,
    pub preshared_key: Option<String>,
    pub allowed_ips: Vec<String>,
    pub endpoint: Option<String>,
    pub persistent_keepalive: Option<u64>,
    pub comment: String,
}
