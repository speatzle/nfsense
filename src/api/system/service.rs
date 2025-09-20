use crate::api::ApiError;
use crate::state::RpcState;
use jsonrpsee::{types::Params, Extensions, RpcModule};
use std::sync::Arc;

const SERVICE_UNBOUND: &str = "unbound.service";
const SERVICE_SSH: &str = "sshd.service";
const SERVICE_KEAV4: &str = "kea-dhcp4-server.service";
const SERVICE_KEAV6: &str = "kea-dhcp6-server.service";
const SERVICE_CHRONY: &str = "chronyd.service";
const SERVICE_NETWORKD: &str = "systemd-networkd.service";
const SERVICE_NFTABLES: &str = "nftables.service";

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_async_method("system.services.status", get_services_status)
        .unwrap();

    module
        .register_async_method("system.services.restart", restart_service)
        .unwrap();

    module
        .register_async_method("system.services.stop", stop_service)
        .unwrap();
}

pub async fn get_services_status<'a>(
    _: Params<'a>,
    _state: Arc<RpcState>,
    _: Extensions,
) -> Result<Vec<String>, ApiError> {
    let res: Vec<String> = vec![
        get_service_status(&_state.dbus_conn, SERVICE_UNBOUND).await?,
        get_service_status(&_state.dbus_conn, SERVICE_SSH).await?,
        get_service_status(&_state.dbus_conn, SERVICE_KEAV4).await?,
        get_service_status(&_state.dbus_conn, SERVICE_KEAV6).await?,
        get_service_status(&_state.dbus_conn, SERVICE_CHRONY).await?,
        get_service_status(&_state.dbus_conn, SERVICE_NETWORKD).await?,
        get_service_status(&_state.dbus_conn, SERVICE_NFTABLES).await?,
    ];
    Ok(res)
}

async fn get_service_status(
    dbus_conn: &zbus::Connection,
    unit: &str,
) -> Result<String, zbus::Error> {
    let systemd_manager = zbus_systemd::systemd1::ManagerProxy::new(dbus_conn).await?;

    match systemd_manager.get_unit(unit.to_string()).await {
        Ok(unit_object_path) => {
            let systemd_unit =
                zbus_systemd::systemd1::UnitProxy::new(dbus_conn, unit_object_path).await?;
            match systemd_unit.active_state().await {
                Ok(status) => Ok(status),
                Err(_err) => Ok("unknown status".to_string()),
            }
        }
        // TODO handle error better
        Err(_err) => Ok("unknown unit".to_string()),
    }
}

// TODO add services whitelist
async fn restart_service<'a>(
    params: Params<'a>,
    _state: Arc<RpcState>,
    _: Extensions,
) -> Result<(), ApiError> {
    let unit: String = params.parse().map_err(ApiError::ParameterDeserialize)?;
    let systemd_manager = zbus_systemd::systemd1::ManagerProxy::new(&_state.dbus_conn).await?;
    let unit_object_path = systemd_manager.get_unit(unit).await?;
    let systemd_unit =
        zbus_systemd::systemd1::UnitProxy::new(&_state.dbus_conn, unit_object_path).await?;
    systemd_unit.restart("replace".to_string()).await?;
    Ok(())
}

async fn stop_service<'a>(
    params: Params<'a>,
    _state: Arc<RpcState>,
    _: Extensions,
) -> Result<(), ApiError> {
    let unit: String = params.parse().map_err(ApiError::ParameterDeserialize)?;
    let systemd_manager = zbus_systemd::systemd1::ManagerProxy::new(&_state.dbus_conn).await?;
    let unit_object_path = systemd_manager.get_unit(unit).await?;
    let systemd_unit =
        zbus_systemd::systemd1::UnitProxy::new(&_state.dbus_conn, unit_object_path).await?;
    systemd_unit.stop("replace".to_string()).await?;
    Ok(())
}
