use std::collections::HashMap;

use jsonrpsee::types::Params;

use crate::{
    definitions::{
        service::{DHCPServer, DNSServer, NTPServer},
        vpn::{WireguardInterface, WireguardPeer},
    },
    state::RpcState,
};

use super::ApiError;

pub fn get_wireguard_status(_: Params, state: &RpcState) -> Result<String, ApiError> {
    Ok("ok".to_string())
}

pub fn get_wireguard_interfaces(
    _: Params,
    state: &RpcState,
) -> Result<HashMap<String, WireguardInterface>, ApiError> {
    Ok(state
        .config_manager
        .get_pending_config()
        .vpn
        .wireguard
        .interfaces)
}

pub fn get_wireguard_peers(
    _: Params,
    state: &RpcState,
) -> Result<HashMap<String, WireguardPeer>, ApiError> {
    Ok(state
        .config_manager
        .get_pending_config()
        .vpn
        .wireguard
        .peers)
}
