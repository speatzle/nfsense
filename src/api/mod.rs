mod config;
mod firewall;
mod network;
mod object;
mod service;
mod system;
mod vpn;

use crate::state::RpcState;
use jsonrpsee::{
    types::{error::ErrorCode, ErrorObject},
    RpcModule,
};

use serde::Deserialize;
use thiserror::Error;
use tracing::info;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Not Implemented")]
    NotImplemented,

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
        match self {
            // Todo Add Proper mappings
            // ApiError::ParameterDeserialize => ErrorCode::InvalidParams,
            ApiError::NotImplemented => ErrorCode::ServerError(0),
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
        .register_method("network.get_static_routes", network::get_static_routes)
        .unwrap();

    module
        .register_method("network.get_interfaces", network::get_interfaces)
        .unwrap();

    module
        .register_method("object.get_services", object::get_services)
        .unwrap();

    module
        .register_method("object.get_addresses", object::get_addresses)
        .unwrap();

    module
        .register_method("service.get_dhcp_servers", service::get_dhcp_servers)
        .unwrap();

    module
        .register_method("service.get_dns_servers", service::get_dns_servers)
        .unwrap();

    module
        .register_method("service.get_ntp_servers", service::get_ntp_servers)
        .unwrap();

    module
        .register_method("firewall.get_forward_rules", firewall::get_forward_rules)
        .unwrap();

    module
        .register_method(
            "firewall.get_destination_nat_rules",
            firewall::get_destination_nat_rules,
        )
        .unwrap();

    module
        .register_method(
            "firewall.get_source_nat_rules",
            firewall::get_source_nat_rules,
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
        .register_method("vpn.get_wireguard_status", vpn::get_wireguard_status)
        .unwrap();

    module
        .register_method(
            "vpn.get_wireguard_interfaces",
            vpn::get_wireguard_interfaces,
        )
        .unwrap();

    module
        .register_method("vpn.get_wireguard_peers", vpn::get_wireguard_peers)
        .unwrap();

    module
}
