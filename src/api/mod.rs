mod network;
mod system;

use crate::state::RpcState;
use jsonrpsee::{
    types::{error::ErrorCode, ErrorObject},
    RpcModule,
};

use custom_error::custom_error;
use tracing::info;

custom_error! { pub ApiError
    InvalidParams = "Invalid Parameters",
    Leet = "1337",
}

impl Into<ErrorObject<'static>> for ApiError {
    fn into(self) -> ErrorObject<'static> {
        match self {
            Self::InvalidParams => ErrorCode::InvalidParams,
            Self::Leet => ErrorCode::ServerError(1337),
            _ => ErrorCode::InternalError,
        }
        .into()
    }
}

pub fn new_rpc_module(state: RpcState) -> RpcModule<RpcState> {
    let mut module = RpcModule::new(state);

    module
        .register_method("ping", |_, _| {
            info!("ping called");
            "pong"
        })
        .unwrap();

    module
        .register_method("system.get_users", system::get_users)
        .unwrap();

    module
        .register_method("network.get_static_routes", network::get_static_routes)
        .unwrap();

    module
}
