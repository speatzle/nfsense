use crate::config_manager::{Change, ChangeAction::Create, ChangeAction::Update};
use crate::delete_thing_by_name;
use crate::{definitions::system::User, state::RpcState};
use jsonrpsee::types::Params;
use jsonrpsee::{Extensions, RpcModule};
use pwhash::sha512_crypt;
use serde::{Deserialize, Serialize};

use crate::api::ApiError;
use ApiError::ConfigError;
use ApiError::HashError;
use ApiError::NotFound;
use ApiError::ParameterDeserialize;

const USER_CHANGE_PATH: &str = "system.user";

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_method("system.users.get", get_user)
        .unwrap();

    module
        .register_method("system.users.list", get_users)
        .unwrap();

    module
        .register_method("system.users.create", create_user)
        .unwrap();

    module
        .register_method("system.users.update", update_user)
        .unwrap();

    module
        .register_method("system.users.delete", delete_thing_by_name!(system.users))
        .unwrap();
}

#[derive(Serialize, Clone)]
pub struct GetUserResult {
    name: String,
    comment: String,
}

#[derive(Deserialize)]
pub struct GetUser {
    name: String,
}

pub fn get_user(p: Params, state: &RpcState, _: &Extensions) -> Result<GetUserResult, ApiError> {
    let u: GetUser = p.parse().map_err(ParameterDeserialize)?;

    let index = state
        .config_manager
        .get_pending_config()
        .system
        .users
        .iter()
        .position(|e| *e.name == u.name);

    match index {
        Some(i) => Ok(GetUserResult {
            name: state.config_manager.get_pending_config().system.users[i]
                .name
                .clone(),
            comment: state.config_manager.get_pending_config().system.users[i]
                .comment
                .clone(),
        }),
        None => Err(NotFound),
    }
}

pub fn get_users(
    _: Params,
    state: &RpcState,
    _: &Extensions,
) -> Result<Vec<GetUserResult>, ApiError> {
    let mut res: Vec<GetUserResult> = Vec::new();
    for u in state
        .config_manager
        .get_pending_config()
        .system
        .users
        .iter()
    {
        res.push(GetUserResult {
            name: u.name.clone(),
            comment: u.comment.clone(),
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

pub fn create_user(p: Params, state: &RpcState, _: &Extensions) -> Result<(), ApiError> {
    let u: CreateUser = p.parse().map_err(ParameterDeserialize)?;

    let hash = sha512_crypt::hash(u.password).map_err(HashError)?;

    let mut cm = state.config_manager.clone();
    let mut tx = cm.start_transaction();

    tx.config.system.users.push(User {
        name: u.name.clone(),
        comment: match u.comment {
            Some(c) => c,
            None => "".to_string(),
        },
        hash: hash,
    });

    tx.commit(Change {
        action: Create,
        path: USER_CHANGE_PATH,
        id: u.name,
    })
    .map_err(ConfigError)
}

#[derive(Deserialize)]
struct UpdateUser {
    name: String,
    thing: CreateUser,
}

pub fn update_user(p: Params, state: &RpcState, _: &Extensions) -> Result<(), ApiError> {
    let u: UpdateUser = p.parse().map_err(ParameterDeserialize)?;

    let mut cm = state.config_manager.clone();
    let mut tx = cm.start_transaction();

    let index = tx
        .config
        .system
        .users
        .iter()
        .position(|e| *e.name == u.name);

    match index {
        Some(i) => {
            let user = &tx.config.system.users[i];

            let hash = if u.thing.password == "" {
                user.hash.clone()
            } else {
                sha512_crypt::hash(u.thing.password).map_err(HashError)?
            };

            tx.config.system.users[i] = User {
                name: u.thing.name,
                comment: match u.thing.comment {
                    Some(c) => c,
                    None => "".to_string(),
                },
                hash,
            };

            tx.commit(Change {
                action: Update,
                path: USER_CHANGE_PATH,
                id: u.name,
            })
            .map_err(ConfigError)
        }
        None => Err(NotFound),
    }
}
