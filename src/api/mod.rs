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

    #[error("Command Error")]
    CommandError,

    //#[error("Already Exists")]
    //AlreadyExists,
    #[error("Hash Error")]
    HashError(#[from] pwhash::error::Error),

    #[error(transparent)]
    ParameterDeserialize(#[from] jsonrpsee::types::ErrorObjectOwned),

    #[error(transparent)]
    ConfigError(#[from] crate::config_manager::ConfigError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    DbusError(#[from] zbus::Error),

    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    UpdateError(#[from] system::update::UpdateError),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}

impl Into<ErrorObject<'static>> for ApiError {
    fn into(self) -> ErrorObject<'static> {
        info!("Converting Error {:?}", self);

        // Todo Add Proper mappings
        ErrorObject::owned::<()>(
            ErrorCode::InternalError.code(),
            self.to_string(),
            None::<()>,
        )
    }
}

#[macro_export]
macro_rules! get_thing_by_name {
    ($( $sub_system:ident ).+) => {
        |params, state, _: &Extensions| {
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
        |params, state, _: &Extensions| {
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
        |_, state, _: &Extensions| {
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
        |params, state, _: &Extensions| {
            let t: $typ = params.parse().map_err(ApiError::ParameterDeserialize)?;

            let mut cm = state.config_manager.clone();
            let mut tx = cm.start_transaction();

            tx.data_mut().$($sub_system).+.push(t);

            // 1. Commit to a temporary vec to get the data changes
            let mut data_changes = Vec::new();
            tx.commit(&mut data_changes).map_err(ApiError::ConfigError)?;

            // 2. If the transaction produced changes, wrap them in a ChangeSet
            if !data_changes.is_empty() {
                let change_set = crate::config_manager::ChangeSet {
                    user: "system".to_string(), // TODO: Get user from session/auth context
                    timestamp: OffsetDateTime::now_utc(),
                    changes: data_changes,
                };
                // 3. Add the complete, user-attributed ChangeSet to the central log
                // NOTE: This assumes `ConfigManager` has been refactored to expose
                // a method to add to the changelog.
                cm.add_to_changelog(change_set);
            }

            Ok(())
        }
    };
}

#[macro_export]
macro_rules! update_thing_by_name {
    ($( $sub_system:ident ).+, $typ:ty) => {
        |params, state, _: &Extensions| {
            use serde::{Deserialize, Serialize};

            #[derive(Deserialize, Serialize)]
            struct UpdateByName {
                name: String,
                thing: $typ,
            }

            let t: UpdateByName = params.parse().map_err(ApiError::ParameterDeserialize)?;
            let mut cm = state.config_manager.clone();
            let mut tx = cm.start_transaction();

            let index = tx.data_mut().$($sub_system).+.iter().position(|e| *e.name == t.name);

            match index {
                Some(i) => {
                    tx.data_mut().$($sub_system).+[i] = t.thing;

                    let mut data_changes = Vec::new();
                    tx.commit(&mut data_changes).map_err(ApiError::ConfigError)?;

                    if !data_changes.is_empty() {
                        let change_set = crate::config_manager::ChangeSet {
                            user: "system".to_string(), // TODO: Get user from session/auth context
                            timestamp: OffsetDateTime::now_utc(),
                            changes: data_changes,
                        };
                        cm.add_to_changelog(change_set);
                    }

                    Ok(())
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
        |params, state, _: &Extensions| {
            use serde::{Deserialize, Serialize};

            #[derive(Deserialize, Serialize)]
            struct UpdateByIndex {
                index: i64,
                thing: $typ,
            }

            let t: UpdateByIndex = params.parse().map_err(ApiError::ParameterDeserialize)?;
            let mut cm = state.config_manager.clone();
            let mut tx = cm.start_transaction();

            if tx.data_mut().$($sub_system).+.len() > t.index as usize {
                tx.data_mut().$($sub_system).+[t.index as usize] = t.thing;

                let mut data_changes = Vec::new();
                tx.commit(&mut data_changes).map_err(ApiError::ConfigError)?;

                if !data_changes.is_empty() {
                    let change_set = crate::config_manager::ChangeSet {
                        user: "system".to_string(), // TODO: Get user from session/auth context
                        timestamp: OffsetDateTime::now_utc(),
                        changes: data_changes,
                    };
                    cm.add_to_changelog(change_set);
                }
                Ok(())
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
        |params, state, _: &Extensions| {
            use serde::{Deserialize, Serialize};

            #[derive(Deserialize, Serialize)]
            struct DeleteByName {
                name: String,
            }

            let t: DeleteByName = params.parse().map_err(ApiError::ParameterDeserialize)?;

            let mut cm = state.config_manager.clone();
            let mut tx = cm.start_transaction();

            let index = tx.data_mut().$($sub_system).+.iter().position(|e| *e.name == t.name);

            match index {
                Some(i) => {
                    tx.data_mut().$($sub_system).+.remove(i);

                    let mut data_changes = Vec::new();
                    tx.commit(&mut data_changes).map_err(ApiError::ConfigError)?;

                    if !data_changes.is_empty() {
                        let change_set = crate::config_manager::ChangeSet {
                            user: "system".to_string(), // TODO: Get user from session/auth context
                            timestamp: OffsetDateTime::now_utc(),
                            changes: data_changes,
                        };
                        cm.add_to_changelog(change_set);
                    }

                    Ok(())
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
        |params, state, _: &Extensions| {
            use serde::{Deserialize, Serialize};

            #[derive(Deserialize, Serialize)]
            struct DeleteByIndex {
                index: i64,
            }

            let t: DeleteByIndex = params.parse().map_err(ApiError::ParameterDeserialize)?;

            let mut cm = state.config_manager.clone();
            let mut tx = cm.start_transaction();

            if tx.data_mut().$($sub_system).+.len() > t.index as usize {
                tx.data_mut().$($sub_system).+.remove(t.index as usize);

                let mut data_changes = Vec::new();
                tx.commit(&mut data_changes).map_err(ApiError::ConfigError)?;

                if !data_changes.is_empty() {
                    let change_set = crate::config_manager::ChangeSet {
                        user: "system".to_string(), // TODO: Get user from session/auth context
                        timestamp: OffsetDateTime::now_utc(),
                        changes: data_changes,
                    };
                    cm.add_to_changelog(change_set);
                }

                Ok(())
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
