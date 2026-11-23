use std::future::Future;

use crate::state::{AppState, RpcState};
use axum::extract::{Request, State};
use axum::response::Response;
use axum::routing::any;
use axum::{Extension, Router};

use futures_util::FutureExt;
use jsonrpsee::core::middleware::{Batch, Notification, RpcServiceBuilder, RpcServiceT};
use jsonrpsee::server::{stop_channel, Server, ServerConfig, ServerHandle};
use jsonrpsee::types::Request as RpcRequest;
use jsonrpsee::types::{ErrorCode, ErrorObject, Id};
use jsonrpsee::MethodResponse;
use jsonrpsee::RpcModule;
use std::panic::AssertUnwindSafe;
use tower::Service;
use tracing::{error, info};

#[derive(Clone)]
pub struct Logger<S> {
    service: S,
}

impl<S> RpcServiceT for Logger<S>
where
    S: RpcServiceT + Send + Sync + Clone + 'static,
{
    type MethodResponse = S::MethodResponse;
    type NotificationResponse = S::NotificationResponse;
    type BatchResponse = S::BatchResponse;

    fn call<'a>(
        &self,
        req: RpcRequest<'a>,
    ) -> impl Future<Output = Self::MethodResponse> + Send + 'a {
        let session = req.extensions().get::<super::auth::Session>();
        match session {
            Some(session) => {
                info!(
                    "rpc call, user: {} method: {}",
                    session.username, req.method
                );
            }
            None => error!("rpc call without session, method: {}", req.method),
        }
        self.service.call(req)
    }

    fn batch<'a>(&self, batch: Batch<'a>) -> impl Future<Output = Self::BatchResponse> + Send + 'a {
        /* TODO fix batch logging
        let session = batch.extensions().get::<super::auth::Session>();
        match session {
            Some(session) => {
                info!("{} rpc batch, user: {}", self.role, session.username);
            }
            None => panic!("rpc batch without session"),
        }
        */
        self.service.batch(batch)
    }
    fn notification<'a>(
        &self,
        n: Notification<'a>,
    ) -> impl Future<Output = Self::NotificationResponse> + Send + 'a {
        self.service.notification(n)
    }
}

/// Turns a panic in any RPC method into an InternalError response, so a single
/// bad request can never take down the connection or the server.
#[derive(Clone)]
pub struct CatchPanic<S> {
    service: S,
}

fn panic_response(id: Id<'static>, method: &str, msg: String) -> MethodResponse {
    error!("rpc method {} panicked: {}", method, msg);
    MethodResponse::error(id, ErrorObject::from(ErrorCode::InternalError))
}

impl<S> RpcServiceT for CatchPanic<S>
where
    S: RpcServiceT<
            MethodResponse = MethodResponse,
            BatchResponse = MethodResponse,
            NotificationResponse = MethodResponse,
        > + Send
        + Sync
        + Clone
        + 'static,
{
    type MethodResponse = MethodResponse;
    type NotificationResponse = MethodResponse;
    type BatchResponse = MethodResponse;

    fn call<'a>(
        &self,
        req: RpcRequest<'a>,
    ) -> impl Future<Output = Self::MethodResponse> + Send + 'a {
        let id = req.id().into_owned();
        let method = req.method_name().to_string();
        // Sync methods run inside call() itself, async ones in the returned future.
        let call = crate::panic::catch(|| self.service.call(req));
        async move {
            match call {
                Ok(fut) => match AssertUnwindSafe(fut).catch_unwind().await {
                    Ok(rp) => rp,
                    Err(p) => panic_response(id, &method, crate::panic::panic_message(&*p)),
                },
                Err(msg) => panic_response(id, &method, msg),
            }
        }
    }

    fn batch<'a>(&self, batch: Batch<'a>) -> impl Future<Output = Self::BatchResponse> + Send + 'a {
        let call = crate::panic::catch(|| self.service.batch(batch));
        async move {
            match call {
                Ok(fut) => match AssertUnwindSafe(fut).catch_unwind().await {
                    Ok(rp) => rp,
                    Err(p) => panic_response(Id::Null, "batch", crate::panic::panic_message(&*p)),
                },
                Err(msg) => panic_response(Id::Null, "batch", msg),
            }
        }
    }

    fn notification<'a>(
        &self,
        n: Notification<'a>,
    ) -> impl Future<Output = Self::NotificationResponse> + Send + 'a {
        let call = crate::panic::catch(|| self.service.notification(n));
        async move {
            let result = match call {
                Ok(fut) => AssertUnwindSafe(fut)
                    .catch_unwind()
                    .await
                    .map_err(|p| crate::panic::panic_message(&*p)),
                Err(msg) => Err(msg),
            };
            result.unwrap_or_else(|msg| {
                error!("rpc notification panicked: {}", msg);
                MethodResponse::notification()
            })
        }
    }
}

pub fn routes(rpc_module: RpcModule<RpcState>) -> (Router<crate::state::AppState>, ServerHandle) {
    // Create stop channel for graceful shutdown
    let (stop_handle, server_handle) = stop_channel();

    // Configure the RPC server
    let server_config = ServerConfig::builder()
        .enable_ws_ping(Default::default())
        .build();

    let rpc_middleware = RpcServiceBuilder::new()
        .layer_fn(|service| CatchPanic { service })
        .layer_fn(|service| Logger { service });

    // Build the tower service for JSON-RPC
    let rpc_service = Server::builder()
        .set_config(server_config)
        .to_service_builder()
        .set_rpc_middleware(rpc_middleware)
        .build(rpc_module, stop_handle);

    // Create a service function that handles both HTTP and WebSocket requests
    let api_handler = move |State(_state): State<AppState>,
                            session: Extension<super::auth::Session>,
                            req: Request| {
        let mut service = rpc_service;
        async move {
            // Call the RPC service which handles both HTTP and WebSocket upgrades
            info!("Handling api connection for user {}", session.username);
            match service.call(req).await {
                Ok(response) => response,
                Err(err) => {
                    error!("RPC service error: {:?}", err);
                    Response::builder()
                        .status(500)
                        .body("Internal Server Error".into())
                        .unwrap()
                }
            }
        }
    };

    (Router::new().route("/api", any(api_handler)), server_handle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;

    #[tokio::test]
    async fn panicking_methods_return_internal_error() {
        let mut module = RpcModule::new(());
        module
            .register_method(
                "sync_panic",
                |_, _, _| -> Result<(), ErrorObject<'static>> { panic!("boom") },
            )
            .unwrap();
        module
            .register_async_method("async_panic", |_, _, _| async move {
                if true {
                    panic!("async boom");
                }
                Ok::<(), ErrorObject<'static>>(())
            })
            .unwrap();
        module.register_method("ok", |_, _, _| "fine").unwrap();

        let (stop_handle, _server_handle) = stop_channel();
        let service = Server::builder()
            .to_service_builder()
            .set_rpc_middleware(RpcServiceBuilder::new().layer_fn(|service| CatchPanic { service }))
            .build(module, stop_handle);

        for (method, expect) in [
            ("sync_panic", "-32603"),
            ("async_panic", "-32603"),
            ("ok", "\"fine\""),
            ("sync_panic", "-32603"),
            ("ok", "\"fine\""),
        ] {
            let req = axum::http::Request::post("/")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{"jsonrpc":"2.0","id":1,"method":"{method}","params":[]}}"#
                )))
                .unwrap();
            let mut svc = service.clone();
            let resp = svc.call(req).await.unwrap();
            let body = axum::body::to_bytes(Body::new(resp.into_body()), 1 << 20)
                .await
                .unwrap();
            let body = String::from_utf8_lossy(&body);
            assert!(body.contains(expect), "{method}: {body}");
        }
    }
}
