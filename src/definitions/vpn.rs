use serde::{Deserialize, Serialize};
use validator::Validate;

// Referencing
use crate::definitions::config::Config;
use crate::definitions::Referenceable;
use crate::definitions::References;
use crate::{impl_referenceable_trait, impl_references_trait};

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct VPN {
    pub wireguard: Wireguard,
}

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct Wireguard {
    pub interfaces: WireguardInterfaces,
    pub peers: WireguardPeers,
}

impl_referenceable_trait!(WireguardInterfaces, WireguardInterface);
impl_references_trait!(
    WireguardInterfaceReference,
    WireguardInterface,
    vpn.wireguard.interfaces
);

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct WireguardInterface {
    pub name: String,
    pub public_key: String,
    pub private_key: String,
    pub listen_port: u64,
    pub peers: Vec<WireguardPeerReference>,
    pub comment: String,
}

impl_referenceable_trait!(WireguardPeers, WireguardPeer);
impl_references_trait!(WireguardPeerReference, WireguardPeer, vpn.wireguard.peers);

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
