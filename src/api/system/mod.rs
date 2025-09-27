use crate::state::RpcState;
use jsonrpsee::RpcModule;

mod log;
mod power;
mod service;
mod user;

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    power::register_methods(module);
    user::register_methods(module);
    service::register_methods(module);
    log::register_methods(module);
}
