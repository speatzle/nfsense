use jsonrpsee::types::Params;

use crate::state::RpcState;

use super::ApiError;

pub fn get_wireguard_status(_: Params, _: &RpcState) -> Result<String, ApiError> {
    Ok("ok".to_string())
}
