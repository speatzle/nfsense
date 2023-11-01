use super::ApiError;
use crate::{
    create_vec_thing,
    definitions::service::{DHCPServer, DNSServer, NTPServer},
    delete_vec_thing, get_vec_thing, list_things,
    state::RpcState,
};
use jsonrpsee::RpcModule;

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_method(
            "service.dhcp_servers.get",
            get_vec_thing!(service.dhcp_servers),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<DHCPServer>, ApiError>, _>(
            "service.dhcp_servers.list",
            list_things!(service.dhcp_servers),
        )
        .unwrap();

    module
        .register_method(
            "service.dhcp_servers.create",
            create_vec_thing!(service.dhcp_servers, DHCPServer),
        )
        .unwrap();

    module
        .register_method(
            "service.dhcp_servers.delete",
            delete_vec_thing!(service.dhcp_servers),
        )
        .unwrap();

    module
        .register_method(
            "service.dns_servers.get",
            get_vec_thing!(service.dns_servers),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<DNSServer>, ApiError>, _>(
            "service.dns_servers.list",
            list_things!(service.dns_servers),
        )
        .unwrap();

    module
        .register_method(
            "service.dns_servers.create",
            create_vec_thing!(service.dns_servers, DNSServer),
        )
        .unwrap();

    module
        .register_method(
            "service.dns_servers.delete",
            delete_vec_thing!(service.dns_servers),
        )
        .unwrap();

    module
        .register_method(
            "service.ntp_servers.get",
            get_vec_thing!(service.ntp_servers),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<NTPServer>, ApiError>, _>(
            "service.ntp_servers.list",
            list_things!(service.ntp_servers),
        )
        .unwrap();

    module
        .register_method(
            "service.ntp_servers.create",
            create_vec_thing!(service.ntp_servers, NTPServer),
        )
        .unwrap();

    module
        .register_method(
            "service.ntp_servers.delete",
            delete_vec_thing!(service.ntp_servers),
        )
        .unwrap();
}
