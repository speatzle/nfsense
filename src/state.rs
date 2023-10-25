use super::config_manager::ConfigManager;
use super::web::auth::SessionState;

#[derive(Clone)]
pub struct AppState {
    pub config_manager: ConfigManager,
    pub session_state: SessionState,
}
