use crate::{definitions::system::User, state::RpcState};
use jsonrpsee::types::Params;
use pwhash::sha512_crypt;
use serde::{Deserialize, Serialize};

use ApiError::ConfigError;
use ApiError::HashError;
use ApiError::NotFound;
use ApiError::ParameterDeserialize;

use super::{ApiError, GetStringID};

#[derive(Serialize, Clone)]
pub struct GetUser {
    name: String,
    comment: String,
}

pub fn get_user(p: Params, state: &RpcState) -> Result<GetUser, ApiError> {
    let u: GetStringID = p.parse().map_err(ParameterDeserialize)?;

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
        None => Err(NotFound),
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
    let u: CreateUser = p.parse().map_err(ParameterDeserialize)?;

    let hash = sha512_crypt::hash(u.password).map_err(HashError)?;

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
        tx.commit().map_err(ConfigError)?;
        Ok(())
    } else {
        tx.revert();
        Err(NotFound)
    }
}

#[derive(Deserialize)]
struct UpdateUser {
    name: String,
    password: String,
    comment: Option<String>,
}

pub fn update_user(p: Params, state: &RpcState) -> Result<(), ApiError> {
    let u: UpdateUser = p.parse().map_err(ParameterDeserialize)?;

    let mut cm = state.config_manager.clone();
    let mut tx = cm.start_transaction();

    match tx.changes.system.users.get(&u.name) {
        Some(user) => {
            // Only Update Password if field is not empty
            let hash = if u.password == "" {
                user.hash.clone()
            } else {
                sha512_crypt::hash(u.password).map_err(HashError)?
            };

            tx.changes.system.users.insert(
                u.name,
                User {
                    comment: match u.comment {
                        Some(c) => c,
                        None => "".to_string(),
                    },
                    hash,
                },
            );
            Ok(())
        }
        None => Err(NotFound),
    }
}

#[derive(Deserialize)]
struct DeleteUser {
    name: String,
}

pub fn delete_user(p: Params, state: &RpcState) -> Result<(), ApiError> {
    let u: DeleteUser = p.parse().map_err(ParameterDeserialize)?;

    let mut cm = state.config_manager.clone();
    let mut tx = cm.start_transaction();

    match tx.changes.system.users.remove(&u.name) {
        Some(_) => tx.commit().map_err(ConfigError),
        None => {
            tx.revert();
            Err(NotFound)
        }
    }
}
