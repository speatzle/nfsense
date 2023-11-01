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
macro_rules! get_map_thing {
    ($( $sub_system:ident ).+) => {
        |params, state| {
            use serde::Deserialize;

            #[derive(Deserialize)]
            struct GetStringID {
                id: String,
            }

            let t: GetStringID = params.parse().map_err(ApiError::ParameterDeserialize)?;

            match state
                .config_manager
                .get_pending_config()
                .$($sub_system).+
                .get(&t.id)
            {
                Some(thing) => Ok(thing.clone()),
                None => Err(ApiError::NotFound),
            }
        }
    };
}

#[macro_export]
macro_rules! get_vec_thing {
    ($( $sub_system:ident ).+) => {
        |params, state| {
            use serde::{Deserialize, Serialize};

            #[derive(Deserialize, Serialize)]
            struct GetIntID {
                id: i64,
            }

            let t: GetIntID = params.parse().map_err(ApiError::ParameterDeserialize)?;
            let things = state
            .config_manager
            .get_pending_config()
            .$($sub_system).+;

            if things.len() > t.id as usize {
                Ok(things[t.id as usize].clone())
            } else {
                Err(ApiError::NotFound)
            }
        }
    };
}

#[macro_export]
macro_rules! get_things {
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
macro_rules! delete_map_thing {
    ($( $sub_system:ident ).+) => {
        |params, state| {
            use serde::{Deserialize, Serialize};

            #[derive(Deserialize, Serialize)]
            struct GetStringID {
                id: String,
            }

            let t: GetStringID = params.parse().map_err(ApiError::ParameterDeserialize)?;

            let mut cm = state.config_manager.clone();
            let mut tx = cm.start_transaction();

            match tx.config.$($sub_system).+.remove(&t.id) {
                Some(_) => tx
                    .commit(crate::config_manager::Change {
                        action: crate::config_manager::ChangeAction::Delete,
                        path: stringify!($($sub_system).+),
                        id: t.id,
                    })
                    .map_err(ApiError::ConfigError),
                None => {
                    tx.revert();
                    Err(ApiError::NotFound)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! delete_vec_thing {
    ($( $sub_system:ident ).+) => {
        |params, state| {
            use serde::{Deserialize, Serialize};

            #[derive(Deserialize, Serialize)]
            struct GetIntID {
                id: i64,
            }

            let t: GetIntID = params.parse().map_err(ApiError::ParameterDeserialize)?;

            let mut cm = state.config_manager.clone();
            let mut tx = cm.start_transaction();

            if tx.config.$($sub_system).+.len() > t.id as usize {
                tx.config.$($sub_system).+.remove(t.id as usize);
                tx.commit(crate::config_manager::Change {
                    action: crate::config_manager::ChangeAction::Delete,
                    path: stringify!($($sub_system).+),
                    id: t.id.to_string(),
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
