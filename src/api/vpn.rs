use std::collections::HashMap;

use super::ApiError;
use crate::definitions::vpn::{WireguardInterface, WireguardPeer};
use crate::state::RpcState;
use crate::{delete_map_thing, get_map_thing, get_things};
use jsonrpsee::types::Params;
use jsonrpsee::RpcModule;

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_method("vpn.wireguard.get_status", get_wireguard_status)
        .unwrap();

    module
        .register_method(
            "vpn.wireguard.get_interface",
            get_map_thing!(vpn.wireguard.interfaces),
        )
        .unwrap();

    module
        .register_method::<Result<HashMap<String, WireguardInterface>, ApiError>, _>(
            "vpn.wireguard.get_interfaces",
            get_things!(vpn.wireguard.interfaces),
        )
        .unwrap();

    module
        .register_method(
            "vpn.wireguard.delete_interface",
            delete_map_thing!(vpn.wireguard.interfaces),
        )
        .unwrap();

    module
        .register_method(
            "vpn.wireguard.get_peer",
            get_map_thing!(vpn.wireguard.peers),
        )
        .unwrap();

    module
        .register_method::<Result<HashMap<String, WireguardPeer>, ApiError>, _>(
            "vpn.wireguard.get_peers",
            get_things!(vpn.wireguard.peers),
        )
        .unwrap();

    module
        .register_method(
            "vpn.wireguard.delete_peer",
            delete_map_thing!(vpn.wireguard.peers),
        )
        .unwrap();
}

pub fn get_wireguard_status(_: Params, _: &RpcState) -> Result<String, ApiError> {
    Ok("ok".to_string())
}
