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
macro_rules! get_thing {
    ($( $sub_system:ident ).+) => {
        |params, state, _: &Extensions| {
            use serde::Deserialize;

            #[derive(Deserialize)]
            struct GetByID {
                id: String,
            }

            let t: GetByID = params.parse().map_err(ApiError::ParameterDeserialize)?;

            let index = state
                .config_manager
                .get_pending_config()
                .$($sub_system).+
                .iter()
                .position(|e| structdb_core::Keyed::get_key(e) == t.id);

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
macro_rules! commit_and_changelog {
    ($cm:ident, $tx:ident, $extensions:ident) => {{
        let mut data_changes = Vec::new();
        $tx.commit(&mut data_changes)
            .map_err(ApiError::ConfigError)?;

        if !data_changes.is_empty() {
            let user = $extensions
                .get::<crate::web::auth::Session>()
                .map(|s| s.username.clone())
                .unwrap_or_else(|| "unknown".to_string());
            let change_set = crate::config_manager::ChangeSet {
                user,
                timestamp: OffsetDateTime::now_utc(),
                changes: data_changes,
            };
            $cm.add_to_changelog(change_set);
        }

        Ok::<(), ApiError>(())
    }};
}

#[macro_export]
macro_rules! create_thing {
    ($( $sub_system:ident ).+, $typ:ty) => {
        |params, state, extensions: &jsonrpsee::Extensions| {
            let t: $typ = params.parse().map_err(ApiError::ParameterDeserialize)?;

            // Extract the key before moving `t` into the collection
            let id = structdb_core::Keyed::get_key(&t);

            let mut cm = state.config_manager.clone();
            let mut tx = cm.start_transaction();

            tx.data_mut().$($sub_system).+.push(t);

            $crate::commit_and_changelog!(cm, tx, extensions)?;

            Ok(id)
        }
    };
}

#[macro_export]
macro_rules! update_thing {
    ($( $sub_system:ident ).+, $typ:ty) => {
        |params, state, extensions: &jsonrpsee::Extensions| {
            use serde::{Deserialize, Serialize};

            #[derive(Deserialize, Serialize)]
            struct UpdateThing {
                id: String,
                thing: $typ,
            }

            let t: UpdateThing = params.parse().map_err(ApiError::ParameterDeserialize)?;
            let mut cm = state.config_manager.clone();
            let mut tx = cm.start_transaction();

            let index = tx.data_mut()
                .$($sub_system).+
                .iter()
                .position(|e| structdb_core::Keyed::get_key(e) == t.id);

            match index {
                Some(i) => {
                    let old_key = structdb_core::Keyed::get_key(&tx.data_mut().$($sub_system).+[i]);
                    let new_key = structdb_core::Keyed::get_key(&t.thing);
                    if old_key != new_key {
                        structdb_core::RenameRefs::rename_refs(
                            tx.data_mut(),
                            stringify!($typ),
                            &old_key,
                            &new_key,
                        );
                    }
                    tx.data_mut().$($sub_system).+[i] = t.thing;

                    $crate::commit_and_changelog!(cm, tx, extensions)?;

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
macro_rules! delete_thing {
    ($( $sub_system:ident ).+) => {
        |params, state, extensions: &jsonrpsee::Extensions| {
            use serde::{Deserialize, Serialize};

            #[derive(Deserialize, Serialize)]
            struct DeleteByID {
                id: String,
            }

            let t: DeleteByID = params.parse().map_err(ApiError::ParameterDeserialize)?;

            let mut cm = state.config_manager.clone();
            let mut tx = cm.start_transaction();

            let index = tx.data_mut().$($sub_system).+.iter().position(|e| structdb_core::Keyed::get_key(e) == t.id);

            delete_thing!(@commit $( $sub_system ).+; index; cm, tx, state, extensions)
        }
    };
    // Internal helper: executes the removal, commit, and changelog logic
    (@commit $( $sub_system:ident ).+; $index:expr; $cm:ident, $tx:ident, $state:ident, $extensions:ident) => {{
        match $index {
            Some(i) => {
                $tx.data_mut().$($sub_system).+.remove(i);

            $crate::commit_and_changelog!($cm, $tx, $extensions)?;

                Ok(())
            }
            None => {
                $tx.revert();
                Err(ApiError::NotFound)
            }
        }
    }};
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
