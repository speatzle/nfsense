use std::collections::HashMap;

use super::ApiError;
use crate::definitions::vpn::{WireguardInterface, WireguardPeer};
use crate::state::RpcState;
use crate::{create_map_thing, delete_map_thing, get_map_thing, list_things, update_map_thing};
use jsonrpsee::types::Params;
use jsonrpsee::RpcModule;

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_method("vpn.wireguard.status", wireguard_status)
        .unwrap();

    module
        .register_method(
            "vpn.wireguard.interfaces.get",
            get_map_thing!(vpn.wireguard.interfaces),
        )
        .unwrap();

    module
        .register_method::<Result<HashMap<String, WireguardInterface>, ApiError>, _>(
            "vpn.wireguard.interfaces.list",
            list_things!(vpn.wireguard.interfaces),
        )
        .unwrap();

    module
        .register_method(
            "vpn.wireguard.interfaces.create",
            create_map_thing!(vpn.wireguard.interfaces, WireguardInterface),
        )
        .unwrap();

    module
        .register_method(
            "vpn.wireguard.interfaces.update",
            update_map_thing!(vpn.wireguard.interfaces, WireguardInterface),
        )
        .unwrap();

    module
        .register_method(
            "vpn.wireguard.interfaces.delete",
            delete_map_thing!(vpn.wireguard.interfaces),
        )
        .unwrap();

    module
        .register_method(
            "vpn.wireguard.peers.get",
            get_map_thing!(vpn.wireguard.peers),
        )
        .unwrap();

    module
        .register_method::<Result<HashMap<String, WireguardPeer>, ApiError>, _>(
            "vpn.wireguard.peers.list",
            list_things!(vpn.wireguard.peers),
        )
        .unwrap();

    module
        .register_method(
            "vpn.wireguard.peers.create",
            create_map_thing!(vpn.wireguard.peers, WireguardPeer),
        )
        .unwrap();

    module
        .register_method(
            "vpn.wireguard.peers.update",
            update_map_thing!(vpn.wireguard.peers, WireguardPeer),
        )
        .unwrap();

    module
        .register_method(
            "vpn.wireguard.peers.delete",
            delete_map_thing!(vpn.wireguard.peers),
        )
        .unwrap();
}

pub fn wireguard_status(_: Params, _: &RpcState) -> Result<String, ApiError> {
    Ok("ok".to_string())
}
