use crate::AppState;
use anyhow::Context;
use axum::routing::post;
use axum::{Json, Router};
use jsonrpsee::core::traits::ToRpcParams;
use jsonrpsee::core::Error;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use axum::{extract::Extension, extract::State, response::IntoResponse};

use tracing::info;

const JSON_RPC_VERSION: &str = "2.0";

// TODO fix this "workaround"
struct ParamConverter {
    params: Option<Box<RawValue>>,
}

impl ToRpcParams for ParamConverter {
    fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, Error> {
        let s = String::from_utf8(serde_json::to_vec(&self.params)?);
        match s {
            Ok(s) => {
                return RawValue::from_string(s)
                    .map(Some)
                    .map_err(Error::ParseError)
            }
            Err(err) => return Err(Error::Custom(err.to_string())),
        }
    }
}

#[derive(Deserialize)]
struct RpcRequest {
    id: i64,
    params: Option<Box<RawValue>>,
    jsonrpc: String,
    method: String,
}

#[derive(Clone, Deserialize, Serialize)]
struct RpcResponse {
    id: i64,
    jsonrpc: String,
    // Note: this Option<Option<>> is for distinguishing between success without a result (null) and and error where the field is skipped
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Option<Box<RawValue>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<RpcErrorObject>,
}

#[derive(Clone, Deserialize, Serialize)]

struct RpcErrorObject {
    code: i64,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Box<RawValue>>,
}

pub fn routes() -> Router<super::super::AppState> {
    Router::new().route("/api", post(api_handler))
}

async fn api_handler(
    State(state): State<AppState>,
    session: Extension<super::auth::Session>,
    body: String,
) -> impl IntoResponse {
    let req: RpcRequest = match serde_json::from_str(&body) {
        Ok(req) => req,
        Err(err) => {
            return Json(RpcResponse {
                id: 0,
                jsonrpc: JSON_RPC_VERSION.to_string(),
                result: None,
                error: Some(RpcErrorObject {
                    // TODO Send back correct code
                    code: 0,
                    message: err.to_string(),
                    data: None,
                }),
            });
        }
    };

    if req.jsonrpc != JSON_RPC_VERSION {
        return Json(RpcResponse {
            id: req.id,
            jsonrpc: JSON_RPC_VERSION.to_string(),
            result: None,
            error: Some(RpcErrorObject {
                // TODO Send back correct code
                code: 0,
                message: "Invalid Jsonrpc Version".to_string(),
                data: None,
            }),
        });
    }
    // TODO check Permissions for method here?

    info!(
        "api hit! user: {:?} method: {:?}",
        session.username, req.method
    );

    // TODO find a async save way to catch panics?
    let res: Result<Option<Box<RawValue>>, Error> = state
        .rpc_module
        .call(&req.method, ParamConverter { params: req.params })
        .await;

    match res {
        Ok(res) => Json(RpcResponse {
            id: req.id,
            jsonrpc: req.jsonrpc,
            result: Some(res),
            error: None,
        }),
        Err(err) => Json(RpcResponse {
            id: req.id,
            jsonrpc: req.jsonrpc,
            result: None,
            error: Some(RpcErrorObject {
                code: 10,
                message: err.to_string(),
                data: None,
            }),
        }),
    }
}
