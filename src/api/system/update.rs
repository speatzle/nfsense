use crate::api::ApiError;
use crate::state::RpcState;
use jsonrpsee::{types::Params, Extensions, RpcModule};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum UpdateError {
    #[error("Target Not Found")]
    TargetNotFound,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateStatus {
    pub current_version: String,
    pub available_updates: Vec<Update>,
    pub job: Option<UpdateJob>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateElement {
    #[serde(rename = "type")]
    pub _type: String,
    pub path: String,
    pub sha256: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Update {
    pub version: String,
    pub newest: bool,
    pub available: bool,
    pub installed: bool,
    pub obsolete: bool,
    pub protected: bool,
    pub incomplete: bool,
    #[serde(alias = "changelogUrls")]
    #[serde(default)]
    pub changelog_urls: Vec<String>,
    #[serde(default)]
    pub content: Vec<UpdateElement>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateJob {
    pub id: u64,
    pub _type: String,
    pub progress: u32,
}

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_async_method("system.update.status", get_update_status)
        .unwrap();
}

pub async fn get_update_status<'a>(
    _: Params<'a>,
    _state: Arc<RpcState>,
    _: Extensions,
) -> Result<UpdateStatus, ApiError> {
    let sysupdate_manager = zbus_systemd::sysupdate1::ManagerProxy::new(&_state.dbus_conn).await?;
    let targets = sysupdate_manager.list_targets().await?;
    for (_class, name, path) in targets {
        if name == "host" {
            let sysupdate_target =
                zbus_systemd::sysupdate1::TargetProxy::new(&_state.dbus_conn, path).await?;
            let mut update_status = UpdateStatus {
                current_version: sysupdate_target.get_version().await?,
                available_updates: vec![],
                job: None,
            };

            let available_updates = sysupdate_target.list(0).await?;
            for update in available_updates {
                update_status.available_updates.push(serde_json::from_str(
                    &sysupdate_target.describe(update.clone(), 0).await?,
                )?);
            }

            let jobs = sysupdate_manager.list_jobs().await?;
            // We assume there can only be one Job
            for (id, _type, progress, _path) in jobs {
                update_status.job = Some(UpdateJob {
                    id,
                    _type,
                    progress,
                });
            }
            return Ok(update_status);
        }
    }

    error!("Did not find Update Target host!");
    Err(UpdateError::TargetNotFound.into())
}
