use super::ApiError;
use crate::{
    create_map_thing,
    definitions::object::{Address, Service},
    delete_map_thing, get_map_thing, list_things,
    state::RpcState,
    update_map_thing,
};
use jsonrpsee::RpcModule;
use std::collections::HashMap;

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_method("object.services.get", get_map_thing!(object.services))
        .unwrap();

    module
        .register_method::<Result<HashMap<String, Service>, ApiError>, _>(
            "object.services.list",
            list_things!(object.services),
        )
        .unwrap();

    module
        .register_method(
            "object.services.create",
            create_map_thing!(object.services, Service),
        )
        .unwrap();

    module
        .register_method(
            "object.services.update",
            update_map_thing!(object.services, Service),
        )
        .unwrap();

    module
        .register_method("object.services.delete", delete_map_thing!(object.services))
        .unwrap();

    module
        .register_method("object.addresses.get", get_map_thing!(object.addresses))
        .unwrap();

    module
        .register_method::<Result<HashMap<String, Address>, ApiError>, _>(
            "object.addresses.list",
            list_things!(object.addresses),
        )
        .unwrap();

    module
        .register_method(
            "object.addresses.create",
            create_map_thing!(object.addresses, Address),
        )
        .unwrap();

    module
        .register_method(
            "object.addresses.update",
            update_map_thing!(object.addresses, Address),
        )
        .unwrap();

    module
        .register_method(
            "object.addresses.delete",
            delete_map_thing!(object.addresses),
        )
        .unwrap();
}
