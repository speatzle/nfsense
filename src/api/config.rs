use jsonrpsee::types::Params;
use jsonrpsee::{Extensions, RpcModule};

use crate::config_manager::ChangeSet;
use crate::definitions::config::Config;
use crate::state::RpcState;

use super::ApiError;
use super::ApiError::ConfigError;

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_method("config.pending.log", get_pending_changelog)
        .unwrap();

    module
        .register_method("config.pending.apply", apply_pending_changes)
        .unwrap();

    module
        .register_method("config.pending.discard", discard_pending_changes)
        .unwrap();

    module
        .register_method("config.pending.get", get_pending_config)
        .unwrap();

    module
        .register_method("config.pending.set", import_pending_config)
        .unwrap();
}

pub fn get_pending_config(_: Params, state: &RpcState, _: &Extensions) -> Result<Config, ApiError> {
    Ok(state.config_manager.clone().get_pending_config())
}

#[derive(serde::Deserialize)]
struct ImportConfigParams {
    config: Config,
}

pub fn import_pending_config(
    p: Params,
    state: &RpcState,
    extensions: &Extensions,
) -> Result<(), ApiError> {
    let params: ImportConfigParams = p.parse().map_err(ApiError::ParameterDeserialize)?;
    let user = extensions
        .get::<crate::web::auth::Session>()
        .map(|s| s.username.clone())
        .unwrap_or_else(|| "unknown".to_string());
    state
        .config_manager
        .clone()
        .set_pending_config(params.config, user)
        .map_err(ConfigError)
}

pub fn get_pending_changelog(
    _: Params,
    state: &RpcState,
    _: &Extensions,
) -> Result<Vec<ChangeSet>, ApiError> {
    Ok(state.config_manager.clone().get_pending_changelog())
}

pub fn apply_pending_changes(_: Params, state: &RpcState, _: &Extensions) -> Result<(), ApiError> {
    state
        .config_manager
        .clone()
        .apply_pending_changes()
        .map_err(ConfigError)
}

pub fn discard_pending_changes(
    _: Params,
    state: &RpcState,
    _: &Extensions,
) -> Result<(), ApiError> {
    state
        .config_manager
        .clone()
        .discard_pending_changes()
        .map_err(ConfigError)
}
