use crate::api::ApiError;
use crate::state::RpcState;
use jsonrpsee::types::Params;
use jsonrpsee::{Extensions, RpcModule};
use std::sync::Arc;

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_async_method("system.power.shutdown", halt_system)
        .unwrap();

    module
        .register_async_method("system.power.restart", reboot_system)
        .unwrap();
}

pub async fn halt_system<'a>(
    _: Params<'a>,
    _state: Arc<RpcState>,
    _: Extensions,
) -> Result<(), ApiError> {
    let systemd_manager = zbus_systemd::systemd1::ManagerProxy::new(&_state.dbus_conn).await?;
    systemd_manager.halt().await?;
    Ok(())
}

pub async fn reboot_system<'a>(
    _: Params<'a>,
    _state: Arc<RpcState>,
    _: Extensions,
) -> Result<(), ApiError> {
    let systemd_manager = zbus_systemd::systemd1::ManagerProxy::new(&_state.dbus_conn).await?;
    systemd_manager.reboot().await?;
    Ok(())
}
