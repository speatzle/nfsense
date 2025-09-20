use super::ApiError;
use crate::definitions::vpn::{WireguardInterface, WireguardPeer};
use crate::state::RpcState;
use crate::{
    create_thing, delete_thing_by_name, get_thing_by_name, list_things, update_thing_by_name,
};
use jsonrpsee::types::Params;
use jsonrpsee::{Extensions, RpcModule};
use std::process::Command;

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_method("vpn.wireguard.status", wireguard_status)
        .unwrap();

    module
        .register_method(
            "vpn.wireguard.interfaces.get",
            get_thing_by_name!(vpn.wireguard.interfaces),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<WireguardInterface>, ApiError>, _>(
            "vpn.wireguard.interfaces.list",
            list_things!(vpn.wireguard.interfaces),
        )
        .unwrap();

    module
        .register_method(
            "vpn.wireguard.interfaces.create",
            create_thing!(vpn.wireguard.interfaces, WireguardInterface),
        )
        .unwrap();

    module
        .register_method(
            "vpn.wireguard.interfaces.update",
            update_thing_by_name!(vpn.wireguard.interfaces, WireguardInterface),
        )
        .unwrap();

    module
        .register_method(
            "vpn.wireguard.interfaces.delete",
            delete_thing_by_name!(vpn.wireguard.interfaces),
        )
        .unwrap();

    module
        .register_method(
            "vpn.wireguard.peers.get",
            get_thing_by_name!(vpn.wireguard.peers),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<WireguardPeer>, ApiError>, _>(
            "vpn.wireguard.peers.list",
            list_things!(vpn.wireguard.peers),
        )
        .unwrap();

    module
        .register_method(
            "vpn.wireguard.peers.create",
            create_thing!(vpn.wireguard.peers, WireguardPeer),
        )
        .unwrap();

    module
        .register_method(
            "vpn.wireguard.peers.update",
            update_thing_by_name!(vpn.wireguard.peers, WireguardPeer),
        )
        .unwrap();

    module
        .register_method(
            "vpn.wireguard.peers.delete",
            delete_thing_by_name!(vpn.wireguard.peers),
        )
        .unwrap();
}

pub fn wireguard_status(_: Params, _: &RpcState, _: &Extensions) -> Result<String, ApiError> {
    match Command::new("wg").output() {
        Ok(out) => Ok(String::from_utf8_lossy(&out.stdout).to_string()),
        Err(err) => Err(ApiError::IOError(err)),
    }
}
