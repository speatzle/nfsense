use crate::AppState;
use axum::routing::post;
use axum::{Json, Router};
use jsonrpsee::core::traits::ToRpcParams;
use jsonrpsee::core::Error;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use axum::{extract::Extension, extract::State, response::IntoResponse};

use tracing::info;

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
            // TODO make this a Parse error wrapping Utf8Error
            Err(err) => return Err(Error::AlreadyStopped),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Box<RawValue>>,
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
    info!("api hit! user: {:?}", session.username);

    // TODO handle Parse Error
    let req: RpcRequest = serde_json::from_str(&body).unwrap();

    // TODO check version

    let params = ParamConverter { params: req.params };

    // TODO check Permissions for method here?

    let res: Result<Option<Box<RawValue>>, Error> =
        state.rpc_module.call(&req.method, params).await;

    match res {
        Ok(res) => Json(RpcResponse {
            id: req.id,
            jsonrpc: req.jsonrpc,
            result: res,
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
