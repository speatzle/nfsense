use std::collections::HashMap;

use jsonrpsee::types::Params;

use crate::{
    definitions::network::{NetworkInterface, StaticRoute},
    state::RpcState,
};

use super::ApiError;

pub fn get_static_routes(_: Params, state: &RpcState) -> Result<Vec<StaticRoute>, ApiError> {
    Ok(state
        .config_manager
        .get_pending_config()
        .network
        .static_routes)
}

pub fn get_interfaces(
    _: Params,
    state: &RpcState,
) -> Result<HashMap<String, NetworkInterface>, ApiError> {
    Ok(state.config_manager.get_pending_config().network.interfaces)
}
