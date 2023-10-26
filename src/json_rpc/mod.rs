// Copied from https://github.com/ralexstokes/axum-json-rpc since it needed minor modifications

use axum::body::HttpBody;
use axum::extract::{FromRequest, FromRequestParts};
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use axum::{BoxError, Json};
use error::{JsonRpcError, JsonRpcErrorReason};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod error;

/// Hack until [try_trait_v2](https://github.com/rust-lang/rust/issues/84277) is not stabilized
pub type JsonRpcResult = Result<JsonRpcResponse, JsonRpcResponse>;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct JsonRpcRequest {
    id: i64,
    jsonrpc: String,
    method: String,
    params: Option<Value>,
}

/// Parses a JSON-RPC request, and returns the request ID, the method name, and the parameters.
/// If the request is invalid, returns an error.
/// ```rust
/// use axum_jrpc::{JsonRpcResult, JsonRpcExtractor, JsonRpcResponse};
///
/// fn router(req: JsonRpcExtractor) -> JsonRpcResult {
///   let req_id = req.get_request_id()?;
///   let method = req.method();
///   match method {
///     "add" => {
///        let params: [i32;2] = req.parse_params()?;
///        return Ok(JsonRpcResponse::success(req_id, params[0] + params[1]))
///     }
///     m =>  Ok(req.method_not_found(m))
///   }
/// }
/// ```
#[derive(Debug)]
pub struct JsonRpcExtractor {
    pub parsed: Value,
    pub method: String,
    pub id: i64,
}

impl JsonRpcExtractor {
    pub fn get_request_id(&self) -> i64 {
        self.id
    }

    pub fn parse_params<T: DeserializeOwned>(self) -> Result<T, JsonRpcResponse> {
        let value = serde_json::from_value(self.parsed);
        match value {
            Ok(v) => Ok(v),
            Err(e) => {
                let error = JsonRpcError::new(
                    JsonRpcErrorReason::InvalidParams,
                    e.to_string(),
                    Value::Null,
                );
                Err(JsonRpcResponse::error(self.id, error))
            }
        }
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn method_not_found(&self, method: &str) -> JsonRpcResponse {
        let error = JsonRpcError::new(
            JsonRpcErrorReason::MethodNotFound,
            format!("Method `{}` not found", method),
            Value::Null,
        );
        JsonRpcResponse::error(self.id, error)
    }
}

#[async_trait::async_trait]
impl<S, B> FromRequest<S, B> for JsonRpcExtractor
where
    S: Send + Sync,
    B: Send + 'static,
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = JsonRpcResponse;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let json = Json::from_request(req, state).await;
        let parsed: JsonRpcRequest = match json {
            Ok(a) => a.0,
            Err(e) => {
                return Err(JsonRpcResponse {
                    id: 0,
                    jsonrpc: "2.0".to_owned(),
                    result: JsonRpcAnswer::Error(JsonRpcError::new(
                        JsonRpcErrorReason::InvalidRequest,
                        e.to_string(),
                        Value::Null,
                    )),
                })
            }
        };
        if parsed.jsonrpc != "2.0" {
            return Err(JsonRpcResponse {
                id: parsed.id,
                jsonrpc: "2.0".to_owned(),
                result: JsonRpcAnswer::Error(JsonRpcError::new(
                    JsonRpcErrorReason::InvalidRequest,
                    "Invalid jsonrpc version".to_owned(),
                    Value::Null,
                )),
            });
        }
        Ok(Self {
            parsed: match parsed.params {
                Some(p) => p,
                None => Value::Null,
            },
            method: parsed.method,
            id: parsed.id,
        })
    }
}

#[derive(Serialize, Debug, Deserialize)]
/// A JSON-RPC response.
pub struct JsonRpcResponse {
    jsonrpc: String,
    pub result: JsonRpcAnswer,
    /// The request ID.
    id: i64,
}

impl JsonRpcResponse {
    /// Returns a response with the given result
    /// Returns JsonRpcError if the `result` is invalid input for [`serde_json::to_value`]
    pub fn success<T: Serialize>(id: i64, result: T) -> Self {
        let result = match serde_json::to_value(result) {
            Ok(v) => v,
            Err(e) => {
                let err = JsonRpcError::new(
                    JsonRpcErrorReason::InternalError,
                    e.to_string(),
                    Value::Null,
                );
                return JsonRpcResponse {
                    id,
                    jsonrpc: "2.0".to_owned(),
                    result: JsonRpcAnswer::Error(err),
                };
            }
        };

        JsonRpcResponse {
            id,
            jsonrpc: "2.0".to_owned(),
            result: JsonRpcAnswer::Result(result),
        }
    }

    pub fn error(id: i64, error: JsonRpcError) -> Self {
        JsonRpcResponse {
            id,
            jsonrpc: "2.0".to_owned(),
            result: JsonRpcAnswer::Error(error),
        }
    }
}

impl IntoResponse for JsonRpcResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(untagged)]
/// JsonRpc [response object](https://www.jsonrpc.org/specification#response_object)
pub enum JsonRpcAnswer {
    Result(Value),
    Error(JsonRpcError),
}
