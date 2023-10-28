use std::collections::HashMap;

use jsonrpsee::types::Params;

use crate::{
    definitions::object::{Address, Service},
    state::RpcState,
};

use super::ApiError;

pub fn get_services(_: Params, state: &RpcState) -> Result<HashMap<String, Service>, ApiError> {
    Ok(state.config_manager.get_pending_config().object.services)
}

pub fn get_addresses(_: Params, state: &RpcState) -> Result<HashMap<String, Address>, ApiError> {
    Ok(state.config_manager.get_pending_config().object.addresses)
}
