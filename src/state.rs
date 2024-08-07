use jsonrpsee::RpcModule;

use super::config_manager::ConfigManager;
use super::web::auth::SessionState;

#[derive(Clone)]
pub struct AppState {
    pub config_manager: ConfigManager,
    pub session_state: SessionState,
    pub rpc_module: RpcModule<RpcState>,
    pub dbus_conn: zbus::Connection,
}

#[derive(Clone)]
pub struct RpcState {
    pub config_manager: ConfigManager,
    pub session_state: SessionState,
    pub dbus_conn: zbus::Connection,
}
