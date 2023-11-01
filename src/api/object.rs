use super::ApiError;
use crate::{
    definitions::object::{Address, Service},
    delete_map_thing, get_map_thing, get_things,
    state::RpcState,
};
use jsonrpsee::RpcModule;
use std::collections::HashMap;

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_method("object.get_service", get_map_thing!(object.services))
        .unwrap();

    module
        .register_method::<Result<HashMap<String, Service>, ApiError>, _>(
            "object.get_services",
            get_things!(object.services),
        )
        .unwrap();

    module
        .register_method("object.delete_service", delete_map_thing!(object.services))
        .unwrap();

    module
        .register_method("object.get_address", get_map_thing!(object.addresses))
        .unwrap();

    module
        .register_method::<Result<HashMap<String, Address>, ApiError>, _>(
            "object.get_addresses",
            get_things!(object.addresses),
        )
        .unwrap();

    module
        .register_method("object.delete_address", delete_map_thing!(object.addresses))
        .unwrap();
}
