use super::ApiError;
use crate::definitions::util::interface::generate_interface_address_objects;
use crate::{
    create_thing,
    definitions::object::{Address, Service},
    delete_thing, get_thing, list_things,
    state::RpcState,
    update_thing,
};
use jsonrpsee::{Extensions, RpcModule};
use time::OffsetDateTime;

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_method("object.services.get", get_thing!(object.services))
        .unwrap();

    module
        .register_method::<Result<Vec<Service>, ApiError>, _>(
            "object.services.list",
            list_things!(object.services),
        )
        .unwrap();

    module
        .register_method::<Result<String, ApiError>, _>(
            "object.services.create",
            create_thing!(object.services, Service),
        )
        .unwrap();

    module
        .register_method::<Result<String, ApiError>, _>(
            "object.services.update",
            update_thing!(object.services, Service),
        )
        .unwrap();

    module
        .register_method::<Result<(), ApiError>, _>(
            "object.services.delete",
            delete_thing!(object.services),
        )
        .unwrap();

    module
        .register_method("object.addresses.get", get_thing!(object.addresses))
        .unwrap();

    module
        .register_method::<Result<Vec<Address>, ApiError>, _>(
            "object.addresses.list",
            |_, state, _: &Extensions| {
                let mut addresses = state
                    .config_manager
                    .get_pending_config()
                    .object
                    .addresses
                    .clone();

                addresses.append(&mut generate_interface_address_objects(state));
                Ok(addresses)
            },
        )
        .unwrap();

    module
        .register_method::<Result<String, ApiError>, _>(
            "object.addresses.create",
            create_thing!(object.addresses, Address),
        )
        .unwrap();

    module
        .register_method::<Result<String, ApiError>, _>(
            "object.addresses.update",
            update_thing!(object.addresses, Address),
        )
        .unwrap();

    module
        .register_method::<Result<(), ApiError>, _>(
            "object.addresses.delete",
            delete_thing!(object.addresses),
        )
        .unwrap();
}
