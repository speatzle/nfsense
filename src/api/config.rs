use jsonrpsee::types::Params;

use crate::config_manager::Change;
use crate::state::RpcState;

use super::ApiError;
use super::ApiError::ConfigError;

pub fn get_pending_changelog(_: Params, state: &RpcState) -> Result<Vec<Change>, ApiError> {
    Ok(state.config_manager.clone().get_pending_changelog())
}

pub fn apply_pending_changes(_: Params, state: &RpcState) -> Result<(), ApiError> {
    state
        .config_manager
        .clone()
        .apply_pending_changes()
        .map_err(ConfigError)
}

pub fn discard_pending_changes(_: Params, state: &RpcState) -> Result<(), ApiError> {
    state
        .config_manager
        .clone()
        .discard_pending_changes()
        .map_err(ConfigError)
}
