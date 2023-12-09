use serde::{Deserialize, Serialize};
use validator::Validate;

use super::firewall;
use super::network;
use super::object;
use super::service;
use super::system;
use super::vpn;
use crate::macro_db;

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

macro_db!(
    {
        [ S: interface, network::StaticRoute, network.interfaces; network.static_routes ()],
        [ S: interface, service::DHCPServer, network.interfaces; service.dhcp_servers ()],
        [ S: interface, service::DNSServer, network.interfaces; service.dns_servers ()],
        [ S: interface, service::NTPServer, network.interfaces; service.ntp_servers ()],
        ->
        network::NetworkInterface
    },
);
