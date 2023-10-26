use std::collections::HashMap;

use super::super::definitions::network::StaticRoute;
use super::super::definitions::system::User;
use super::super::AppState;
use axum::routing::post;
use axum::{Json, Router};
use jsonrpsee::types::Params;
use tower_cookies::{Cookie, Cookies};

use axum::{
    extract::Extension,
    extract::State,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
};

use jsonrpsee::server::{RpcModule, Server};

use tracing::info;

use custom_error::custom_error;

custom_error! { ApiError
    BadRequest = "Bad Request Parameters",
}

struct RpcRequest<'a> {
    id: i64,
    params: Params<'a>,
    jsonrpc: String,
    method: String,
}

struct RpcResponse<'a> {
    id: i64,
    payload: Params<'a>,
    jsonrpc: String,
}

pub fn routes() -> Router<super::super::AppState> {
    Router::new().route("/api", post(api_handler))
}

async fn api_handler(
    State(state): State<AppState>,
    session: Extension<super::auth::Session>,
    Json(rpc_request): Json<RpcRequest<'_>>,
) -> impl IntoResponse {
    info!("api hit! user: {:?}", session.username);
    let module = RpcModule::new(state);
    module
        .register_method("say_hello", |_, _| {
            println!("say_hello method called!");
            "Hello there!!"
        })
        .unwrap();

    module
        .register_method("System.GetUsers", get_users)
        .unwrap();
    module
        .register_method("Network.GetStaticRoutes", get_static_routes)
        .unwrap();

    let res = module.call(&rpc_request.method, rpc_request.params).await;

    match res {
        Ok(res) => RpcResponse {
            id: rpc_request.id,
            jsonrpc: rpc_request.jsonrpc,
            payload: res,
        },
        // TODO make Error Response
        Err(err) => RpcResponse {
            id: rpc_request.id,
            jsonrpc: rpc_request.jsonrpc,
            payload: res,
        },
    }
}

fn get_users(_: Params, state: &AppState) -> Result<HashMap<String, User>, String> {
    Ok(state.config_manager.get_pending_config().system.users)
}

fn get_static_routes(_: Params, state: &AppState) -> Result<Vec<StaticRoute>, String> {
    Ok(state
        .config_manager
        .get_pending_config()
        .network
        .static_routes)
}
