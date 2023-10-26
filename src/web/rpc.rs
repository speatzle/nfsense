use super::super::json_rpc::{JsonRpcExtractor, JsonRpcResponse, JsonRpcResult};
use super::super::AppState;
use axum::routing::post;
use axum::{Json, Router};
use tower_cookies::{Cookie, Cookies};

use axum::{
    extract::Extension,
    extract::State,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
};

use tracing::info;

pub fn routes() -> Router<super::super::AppState> {
    Router::new().route("/api", post(api_handler))
}

async fn api_handler(
    State(state): State<AppState>,
    session: Extension<super::auth::Session>,
    req: JsonRpcExtractor,
) -> JsonRpcResult {
    info!("api hit! user: {:?}", session.username);
    let req_id = req.get_request_id();
    let method = req.method();
    let response = match method {
        "add" => {
            let params: [i32; 2] = req.parse_params()?;
            JsonRpcResponse::success(req_id, params[0] + params[1])
        }
        "System.GetUsers" => JsonRpcResponse::success(
            req_id,
            state.config_manager.get_pending_config().system.users,
        ),
        m => req.method_not_found(m),
    };

    Ok(response)
}
