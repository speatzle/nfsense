use std::collections::HashMap;

use crate::{definitions::system::User, state::RpcState};
use jsonrpsee::types::Params;

use super::ApiError;

pub fn get_users(_: Params, state: &RpcState) -> Result<HashMap<String, User>, ApiError> {
    Ok(state.config_manager.get_pending_config().system.users)
}
