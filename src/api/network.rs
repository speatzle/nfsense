use super::ApiError;
use crate::{
    definitions::network::{NetworkInterface, StaticRoute},
    delete_map_thing, delete_vec_thing, get_map_thing, get_things, get_vec_thing,
    state::RpcState,
};
use jsonrpsee::RpcModule;
use std::collections::HashMap;

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_method(
            "network.get_static_route",
            get_vec_thing!(network.static_routes),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<StaticRoute>, ApiError>, _>(
            "network.get_static_routes",
            get_things!(network.static_routes),
        )
        .unwrap();

    module
        .register_method(
            "network.delete_static_route",
            delete_vec_thing!(network.static_routes),
        )
        .unwrap();

    module
        .register_method("network.get_interface", get_map_thing!(network.interfaces))
        .unwrap();

    module
        .register_method::<Result<HashMap<String, NetworkInterface>, ApiError>, _>(
            "network.get_interfaces",
            get_things!(network.interfaces),
        )
        .unwrap();

    module
        .register_method(
            "network.delete_interface",
            delete_map_thing!(network.interfaces),
        )
        .unwrap();
}
