use std::future::Future;

use crate::state::{AppState, RpcState};
use axum::extract::{Request, State};
use axum::response::Response;
use axum::routing::any;
use axum::{Extension, Router};

use jsonrpsee::core::middleware::{Batch, Notification, RpcServiceBuilder, RpcServiceT};
use jsonrpsee::server::{stop_channel, Server, ServerConfig};
use jsonrpsee::types::Request as RpcRequest;
use jsonrpsee::RpcModule;
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
            None => panic!("rpc call without session"),
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

pub fn routes(rpc_module: RpcModule<RpcState>) -> Router<super::super::AppState> {
    // Create stop channel for graceful shutdown
    let (stop_handle, _server_handle) = stop_channel();

    // Configure the RPC server
    let server_config = ServerConfig::builder()
        .enable_ws_ping(Default::default())
        .build();

    let rpc_middleware = RpcServiceBuilder::new().layer_fn(|service| Logger { service });

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

    Router::new().route("/api", any(api_handler))
}
