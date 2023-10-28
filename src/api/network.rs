use jsonrpsee::types::Params;

use crate::{definitions::network::StaticRoute, state::RpcState};

use super::ApiError;

pub fn get_static_routes(_: Params, state: &RpcState) -> Result<Vec<StaticRoute>, ApiError> {
    Ok(state
        .config_manager
        .get_pending_config()
        .network
        .static_routes)
}
