mod config;
mod firewall;
mod network;
mod object;
mod service;
mod system;
mod vpn;

use crate::state::RpcState;
use jsonrpsee::{
    types::{error::ErrorCode, ErrorObject},
    RpcModule,
};
use thiserror::Error;
use tracing::info;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,

    //#[error("Already Exists")]
    //AlreadyExists,
    #[error("Hash Error")]
    HashError(#[from] pwhash::error::Error),

    #[error(transparent)]
    ParameterDeserialize(#[from] jsonrpsee::types::ErrorObjectOwned),

    #[error(transparent)]
    ConfigError(#[from] crate::config_manager::ConfigError),
}

impl Into<ErrorObject<'static>> for ApiError {
    fn into(self) -> ErrorObject<'static> {
        info!("Converting Error {:?}", self);
        match self {
            // Todo Add Proper mappings
            _ => ErrorCode::InternalError,
        }
        .into()
    }
}

#[macro_export]
macro_rules! get_thing_by_name {
    ($( $sub_system:ident ).+) => {
        |params, state| {
            use serde::Deserialize;

            #[derive(Deserialize)]
            struct GetByName {
                name: String,
            }

            let t: GetByName = params.parse().map_err(ApiError::ParameterDeserialize)?;

            let index = state
            .config_manager
            .get_pending_config()
            .$($sub_system).+.iter().position(|e| *e.name == t.name);

            match index {
                Some(i) => Ok(state
                        .config_manager
                        .get_pending_config()
                        .$($sub_system).+[i].clone()),
                None => Err(ApiError::NotFound)
            }
        }
    };
}

#[macro_export]
macro_rules! get_thing_by_index {
    ($( $sub_system:ident ).+) => {
        |params, state| {
            use serde::{Deserialize, Serialize};

            #[derive(Deserialize, Serialize)]
            struct GetByIndex {
                index: i64,
            }

            let t: GetByIndex = params.parse().map_err(ApiError::ParameterDeserialize)?;
            let things = state
            .config_manager
            .get_pending_config()
            .$($sub_system).+;

            if things.len() > t.index as usize {
                Ok(things[t.index as usize].clone())
            } else {
                Err(ApiError::NotFound)
            }
        }
    };
}

#[macro_export]
macro_rules! list_things {
    ($( $sub_system:ident ).+) => {
        |_, state| {
            Ok(state
            .config_manager
            .get_pending_config()
            .$($sub_system).+)
        }
    };
}

#[macro_export]
macro_rules! create_thing {
    ($( $sub_system:ident ).+, $typ:ty) => {
        |params, state| {
            let t: $typ = params.parse().map_err(ApiError::ParameterDeserialize)?;

            let mut cm = state.config_manager.clone();
            let mut tx = cm.start_transaction();

            tx.config.$($sub_system).+.push(t);
            let id = {tx.config.$($sub_system).+.len() - 1}.to_string();
            tx.commit(crate::config_manager::Change {
                action: crate::config_manager::ChangeAction::Create,
                path: stringify!($($sub_system).+),
                id,
            })
            .map_err(ApiError::ConfigError)
        }
    };
}

#[macro_export]
macro_rules! update_thing_by_name {
    ($( $sub_system:ident ).+, $typ:ty) => {
        |params, state| {
            use serde::{Deserialize, Serialize};

            #[derive(Deserialize, Serialize)]
            struct UpdateByName {
                name: String,
                thing: $typ
            }

            let t: UpdateByName = params.parse().map_err(ApiError::ParameterDeserialize)?;
            let mut cm = state.config_manager.clone();
            let mut tx = cm.start_transaction();

            let index = tx.config.$($sub_system).+.iter().position(|e| *e.name == t.name);

            match index {
                Some(i) => {
                    tx.config.$($sub_system).+[i] = t.thing;

                    tx.commit(crate::config_manager::Change {
                        action: crate::config_manager::ChangeAction::Update,
                        path: stringify!($($sub_system).+),
                        id: t.name,
                    })
                    .map_err(ApiError::ConfigError)
                }
                None => {
                    tx.revert();
                    Err(ApiError::NotFound)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! update_thing_by_index {
    ($( $sub_system:ident ).+, $typ:ty) => {
        |params, state| {
            use serde::{Deserialize, Serialize};

            #[derive(Deserialize, Serialize)]
            struct UpdateByIndex {
                index: i64,
                thing: $typ
            }

            let t: UpdateByIndex = params.parse().map_err(ApiError::ParameterDeserialize)?;
            let mut cm = state.config_manager.clone();
            let mut tx = cm.start_transaction();

            if tx.config.$($sub_system).+.len() > t.index as usize {
                tx.config.$($sub_system).+[t.index as usize] = t.thing;

                tx.commit(crate::config_manager::Change {
                    action: crate::config_manager::ChangeAction::Update,
                    path: stringify!($($sub_system).+),
                    id: t.index.to_string(),
                })
                .map_err(ApiError::ConfigError)
            } else {
                tx.revert();
                Err(ApiError::NotFound)
            }
        }
    };
}

#[macro_export]
macro_rules! delete_thing_by_name {
    ($( $sub_system:ident ).+) => {
        |params, state| {
            use serde::{Deserialize, Serialize};

            #[derive(Deserialize, Serialize)]
            struct DeleteByName {
                name: String,
            }

            let t: DeleteByName = params.parse().map_err(ApiError::ParameterDeserialize)?;

            let mut cm = state.config_manager.clone();
            let mut tx = cm.start_transaction();

            let index = tx.config.$($sub_system).+.iter().position(|e| *e.name == t.name);

            match index {
                Some(i) => {
                    tx.config.$($sub_system).+.remove(i);

                    tx.commit(crate::config_manager::Change {
                        action: crate::config_manager::ChangeAction::Delete,
                        path: stringify!($($sub_system).+),
                        id: t.name,
                    })
                    .map_err(ApiError::ConfigError)
                }
                None => {
                    tx.revert();
                    Err(ApiError::NotFound)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! delete_thing_by_index {
    ($( $sub_system:ident ).+) => {
        |params, state| {
            use serde::{Deserialize, Serialize};

            #[derive(Deserialize, Serialize)]
            struct DeleteByIndex {
                index: i64,
            }

            let t: DeleteByIndex = params.parse().map_err(ApiError::ParameterDeserialize)?;

            let mut cm = state.config_manager.clone();
            let mut tx = cm.start_transaction();

            if tx.config.$($sub_system).+.len() > t.index as usize {
                tx.config.$($sub_system).+.remove(t.index as usize);
                tx.commit(crate::config_manager::Change {
                    action: crate::config_manager::ChangeAction::Delete,
                    path: stringify!($($sub_system).+),
                    id: t.index.to_string(),
                })
                .map_err(ApiError::ConfigError)
            } else {
                tx.revert();
                Err(ApiError::NotFound)
            }
        }
    };
}

pub fn new_rpc_module(state: RpcState) -> RpcModule<RpcState> {
    let mut module = RpcModule::new(state);

    config::register_methods(&mut module);
    firewall::register_methods(&mut module);
    network::register_methods(&mut module);
    object::register_methods(&mut module);
    service::register_methods(&mut module);
    system::register_methods(&mut module);
    vpn::register_methods(&mut module);

    module
}
