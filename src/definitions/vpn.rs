use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct VPN {
    #[garde(dive)]
    pub wireguard: Wireguard,
}

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct Wireguard {
    #[garde(dive)]
    pub interfaces: Vec<WireguardInterface>,
    #[garde(dive)]
    pub peers: Vec<WireguardPeer>,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
#[garde(allow_unvalidated)]
pub struct WireguardInterface {
    pub name: String,
    pub public_key: String,
    pub private_key: String,
    pub listen_port: u64,
    pub peers: Vec<String>,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
#[garde(allow_unvalidated)]
pub struct WireguardPeer {
    pub name: String,
    pub public_key: String,
    pub preshared_key: Option<String>,
    pub allowed_ips: Vec<String>,
    pub endpoint: Option<String>,
    pub persistent_keepalive: Option<u64>,
    pub comment: String,
}
