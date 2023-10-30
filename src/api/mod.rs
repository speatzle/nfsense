mod config;
mod firewall;
mod network;
mod object;
mod service;
mod system;
mod vpn;

use std::collections::HashMap;

use crate::{
    definitions::{
        firewall::{DestinationNATRule, ForwardRule, SourceNATRule},
        network::{NetworkInterface, StaticRoute},
        object::{Address, Service},
        service::{DHCPServer, DNSServer, NTPServer},
        vpn::{WireguardInterface, WireguardPeer},
    },
    state::RpcState,
};
use jsonrpsee::{
    types::{error::ErrorCode, ErrorObject},
    RpcModule,
};

use serde::Deserialize;
use thiserror::Error;
use tracing::info;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,

    #[error("Hash Error")]
    HashError(#[from] pwhash::error::Error),

    #[error(transparent)]
    ParameterDeserialize(#[from] jsonrpsee::types::ErrorObjectOwned),

    #[error(transparent)]
    ConfigError(#[from] crate::config_manager::ConfigError),
}

impl Into<ErrorObject<'static>> for ApiError {
    fn into(self) -> ErrorObject<'static> {
        info!("Converting Error {:?}", self);
        match self {
            // Todo Add Proper mappings
            _ => ErrorCode::InternalError,
        }
        .into()
    }
}

#[derive(Deserialize)]
struct GetStringID {
    id: String,
}

#[derive(Deserialize)]
struct GetIntID {
    id: i64,
}

#[macro_export]
macro_rules! get_map_thing {
    ($( $sub_system:ident ).+) => {
        |params, state| {
            let t: GetStringID = params.parse().map_err(ApiError::ParameterDeserialize)?;

            match state
                .config_manager
                .get_pending_config()
                .$($sub_system).+
                .get(&t.id)
            {
                Some(thing) => Ok(thing.clone()),
                None => Err(ApiError::NotFound),
            }
        }
    };
}

#[macro_export]
macro_rules! get_vec_thing {
    ($( $sub_system:ident ).+) => {
        |params, state| {
            let t: GetIntID = params.parse().map_err(ApiError::ParameterDeserialize)?;
            let things = state
            .config_manager
            .get_pending_config()
            .$($sub_system).+;

            if things.len() > t.id as usize{
                Ok(things[t.id as usize].clone())
            } else {
                Err(ApiError::NotFound)
            }
        }
    };
}

#[macro_export]
macro_rules! get_things {
    ($( $sub_system:ident ).+) => {
        |_, state| {
            Ok(state
            .config_manager
            .get_pending_config()
            .$($sub_system).+)
        }
    };
}

pub fn new_rpc_module(state: RpcState) -> RpcModule<RpcState> {
    let mut module = RpcModule::new(state);

    module
        .register_method("system.get_user", system::get_user)
        .unwrap();

    module
        .register_method("system.get_users", system::get_users)
        .unwrap();

    module
        .register_method("system.create_user", system::create_user)
        .unwrap();

    module
        .register_method("system.update_user", system::update_user)
        .unwrap();

    module
        .register_method("system.delete_user", system::delete_user)
        .unwrap();

    module
        .register_method(
            "network.get_static_route",
            get_vec_thing!(network.static_routes),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<StaticRoute>, ApiError>, _>(
            "network.get_static_routes",
            get_things!(network.static_routes),
        )
        .unwrap();

    module
        .register_method("network.get_interface", get_map_thing!(network.interfaces))
        .unwrap();

    module
        .register_method::<Result<HashMap<String, NetworkInterface>, ApiError>, _>(
            "network.get_interfaces",
            get_things!(network.interfaces),
        )
        .unwrap();

    module
        .register_method("object.get_service", get_map_thing!(object.services))
        .unwrap();

    module
        .register_method::<Result<HashMap<String, Service>, ApiError>, _>(
            "object.get_services",
            get_things!(object.services),
        )
        .unwrap();

    module
        .register_method("object.get_address", get_map_thing!(object.addresses))
        .unwrap();

    module
        .register_method::<Result<HashMap<String, Address>, ApiError>, _>(
            "object.get_addresses",
            get_things!(object.addresses),
        )
        .unwrap();

    module
        .register_method(
            "service.get_dhcp_server",
            get_vec_thing!(service.dhcp_servers),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<DHCPServer>, ApiError>, _>(
            "service.get_dhcp_servers",
            get_things!(service.dhcp_servers),
        )
        .unwrap();

    module
        .register_method(
            "service.get_dns_server",
            get_vec_thing!(service.dns_servers),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<DNSServer>, ApiError>, _>(
            "service.get_dns_servers",
            get_things!(service.dns_servers),
        )
        .unwrap();

    module
        .register_method(
            "service.get_ntp_server",
            get_vec_thing!(service.ntp_servers),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<NTPServer>, ApiError>, _>(
            "service.get_ntp_servers",
            get_things!(service.ntp_servers),
        )
        .unwrap();

    module
        .register_method(
            "firewall.get_forward_rule",
            get_vec_thing!(firewall.forward_rules),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<ForwardRule>, ApiError>, _>(
            "firewall.get_forward_rules",
            get_things!(firewall.forward_rules),
        )
        .unwrap();

    module
        .register_method(
            "firewall.get_destination_nat_rule",
            get_vec_thing!(firewall.destination_nat_rules),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<DestinationNATRule>, ApiError>, _>(
            "firewall.get_destination_nat_rules",
            get_things!(firewall.destination_nat_rules),
        )
        .unwrap();

    module
        .register_method(
            "firewall.get_source_nat_rule",
            get_vec_thing!(firewall.source_nat_rules),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<SourceNATRule>, ApiError>, _>(
            "firewall.get_source_nat_rules",
            get_things!(firewall.source_nat_rules),
        )
        .unwrap();

    module
        .register_method(
            "config.get_pending_changelog",
            config::get_pending_changelog,
        )
        .unwrap();

    module
        .register_method(
            "config.apply_pending_changes",
            config::apply_pending_changes,
        )
        .unwrap();

    module
        .register_method(
            "config.discard_pending_changes",
            config::discard_pending_changes,
        )
        .unwrap();

    module
        .register_method("vpn.wireguard.get_status", vpn::get_wireguard_status)
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
}
