use jsonrpsee::types::Params;

use crate::state::RpcState;

use super::ApiError;

pub fn get_pending_changelog(_: Params, state: &RpcState) -> Result<(), ApiError> {
    Err(ApiError::Leet)
}

pub fn apply_pending_changes(_: Params, state: &RpcState) -> Result<(), ApiError> {
    state
        .config_manager
        .clone()
        .apply_pending_changes()
        .map_err(|source| ApiError::Leet)
}

pub fn discard_pending_changes(_: Params, state: &RpcState) -> Result<(), ApiError> {
    state
        .config_manager
        .clone()
        .discard_pending_changes()
        .map_err(|source| ApiError::Leet)
}
