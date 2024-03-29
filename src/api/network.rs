use super::ApiError;
use crate::{
    create_thing,
    definitions::network::{Link, NetworkInterface, StaticRoute},
    delete_thing_by_index, delete_thing_by_name, get_thing_by_index, get_thing_by_name,
    list_things,
    state::RpcState,
    update_thing_by_index, update_thing_by_name,
};
use jsonrpsee::{types::Params, RpcModule};
use std::process::Command;

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_method(
            "network.static_routes.get",
            get_thing_by_index!(network.static_routes),
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
            create_thing!(network.static_routes, StaticRoute),
        )
        .unwrap();

    module
        .register_method(
            "network.static_routes.update",
            update_thing_by_index!(network.static_routes, StaticRoute),
        )
        .unwrap();

    module
        .register_method(
            "network.static_routes.delete",
            delete_thing_by_index!(network.static_routes),
        )
        .unwrap();

    module
        .register_method(
            "network.interfaces.get",
            get_thing_by_name!(network.interfaces),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<NetworkInterface>, ApiError>, _>(
            "network.interfaces.list",
            list_things!(network.interfaces),
        )
        .unwrap();

    module
        .register_method(
            "network.interfaces.create",
            create_thing!(network.interfaces, NetworkInterface),
        )
        .unwrap();

    module
        .register_method(
            "network.interfaces.update",
            update_thing_by_name!(network.interfaces, NetworkInterface),
        )
        .unwrap();

    module
        .register_method(
            "network.interfaces.delete",
            delete_thing_by_name!(network.interfaces),
        )
        .unwrap();
    module
        .register_method("network.links.list", list_network_links)
        .unwrap();
}

pub fn list_network_links(_: Params, _state: &RpcState) -> Result<Vec<Link>, ApiError> {
    match Command::new("ls")
        .arg("-w")
        .arg("1")
        .arg("/sys/class/net")
        .output()
    {
        Ok(out) => {
            let mut links = vec![];
            let output = String::from_utf8_lossy(&out.stdout).to_string();

            for l in output.split("\n") {
                links.push(Link {
                    name: l.to_string(),
                })
            }

            Ok(links)
        }
        Err(err) => Err(ApiError::IOError(err)),
    }
}
