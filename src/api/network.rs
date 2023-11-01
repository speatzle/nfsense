use super::ApiError;
use crate::{
    create_vec_thing,
    definitions::network::{NetworkInterface, StaticRoute},
    delete_map_thing, delete_vec_thing, get_map_thing, get_vec_thing, list_things,
    state::RpcState,
};
use jsonrpsee::RpcModule;
use std::collections::HashMap;

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_method(
            "network.static_routes.get",
            get_vec_thing!(network.static_routes),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<StaticRoute>, ApiError>, _>(
            "network.static_routes.list",
            list_things!(network.static_routes),
        )
        .unwrap();

    module
        .register_method(
            "network.static_routes.create",
            create_vec_thing!(network.static_routes, StaticRoute),
        )
        .unwrap();

    module
        .register_method(
            "network.static_routes.delete",
            delete_vec_thing!(network.static_routes),
        )
        .unwrap();

    module
        .register_method("network.interfaces.get", get_map_thing!(network.interfaces))
        .unwrap();

    module
        .register_method::<Result<HashMap<String, NetworkInterface>, ApiError>, _>(
            "network.interfaces.list",
            list_things!(network.interfaces),
        )
        .unwrap();

    module
        .register_method(
            "network.interfaces.delete",
            delete_map_thing!(network.interfaces),
        )
        .unwrap();
}
