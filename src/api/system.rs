use std::collections::HashMap;

use crate::{definitions::system::User, state::RpcState};
use jsonrpsee::types::Params;
use pwhash::sha512_crypt;
use serde::Deserialize;

use super::ApiError;

pub fn get_users(_: Params, state: &RpcState) -> Result<HashMap<String, User>, ApiError> {
    Ok(state.config_manager.get_pending_config().system.users)
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    password: String,
    comment: String,
}

pub fn create_user(p: Params, state: &RpcState) -> Result<(), ApiError> {
    let u: CreateUser = p.parse().unwrap();

    let hash = sha512_crypt::hash(u.password).unwrap();

    let mut cm = state.config_manager.clone();
    let mut tx = cm.start_transaction();

    if tx
        .changes
        .system
        .users
        .insert(
            u.name,
            User {
                comment: u.comment,
                hash: hash,
            },
        )
        .is_none()
    {
        return tx.commit().map_err(|source| ApiError::InvalidParams);
    } else {
        tx.revert();
        Err(ApiError::InvalidParams)
    }
}
