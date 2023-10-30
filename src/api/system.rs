use std::collections::HashMap;

use crate::{definitions::system::User, state::RpcState};
use jsonrpsee::types::Params;
use pwhash::sha512_crypt;
use serde::{Deserialize, Serialize};

use super::{ApiError, GetStringID};

#[derive(Serialize, Clone)]
pub struct GetUser {
    name: String,
    comment: String,
}

pub fn get_user(p: Params, state: &RpcState) -> Result<GetUser, ApiError> {
    let u: GetStringID = p.parse().unwrap();

    match state
        .config_manager
        .get_pending_config()
        .system
        .users
        .get(&u.id)
    {
        Some(user) => Ok(GetUser {
            name: u.id,
            comment: user.comment.clone(),
        }),
        None => Err(ApiError::InvalidParams),
    }
}

pub fn get_users(_: Params, state: &RpcState) -> Result<Vec<GetUser>, ApiError> {
    let mut res: Vec<GetUser> = Vec::new();
    for u in state
        .config_manager
        .get_pending_config()
        .system
        .users
        .iter()
    {
        res.push(GetUser {
            name: u.0.to_string(),
            comment: u.1.comment.clone(),
        })
    }

    Ok(res)
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    password: String,
    comment: Option<String>,
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
                comment: match u.comment {
                    Some(c) => c,
                    None => "".to_string(),
                },
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

#[derive(Deserialize)]
struct UpdateUser {
    name: String,
    password: String,
    comment: Option<String>,
}

pub fn update_user(p: Params, state: &RpcState) -> Result<(), ApiError> {
    let u: UpdateUser = p.parse().unwrap();

    let mut cm = state.config_manager.clone();
    let mut tx = cm.start_transaction();

    match tx.changes.system.users.get(&u.name) {
        Some(user) => {
            tx.changes.system.users.insert(
                u.name,
                User {
                    comment: match u.comment {
                        Some(c) => c,
                        None => "".to_string(),
                    },
                    // Only Update Password if field is not empty
                    hash: if u.password == "" {
                        user.hash.clone()
                    } else {
                        sha512_crypt::hash(u.password).unwrap()
                    },
                },
            );
            Ok(())
        }
        None => Err(ApiError::InvalidParams),
    }
}

#[derive(Deserialize)]
struct DeleteUser {
    name: String,
}

pub fn delete_user(p: Params, state: &RpcState) -> Result<(), ApiError> {
    let u: DeleteUser = p.parse().unwrap();

    let mut cm = state.config_manager.clone();
    let mut tx = cm.start_transaction();

    match tx.changes.system.users.remove(&u.name) {
        Some(_) => return tx.commit().map_err(|source| ApiError::InvalidParams),
        None => {
            tx.revert();
            Err(ApiError::InvalidParams)
        }
    }
}
