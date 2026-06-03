use super::ApiError;
use crate::{
    create_thing,
    definitions::service::{DHCPServer, DNSServer, NTPServer},
    delete_thing, get_thing, list_things,
    state::RpcState,
    update_thing,
};
use jsonrpsee::{Extensions, RpcModule};
use time::OffsetDateTime;

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_method("service.dhcp_servers.get", get_thing!(service.dhcp_servers))
        .unwrap();

    module
        .register_method::<Result<Vec<DHCPServer>, ApiError>, _>(
            "service.dhcp_servers.list",
            list_things!(service.dhcp_servers),
        )
        .unwrap();

    module
        .register_method::<Result<String, ApiError>, _>(
            "service.dhcp_servers.create",
            create_thing!(service.dhcp_servers, DHCPServer),
        )
        .unwrap();

    module
        .register_method::<Result<String, ApiError>, _>(
            "service.dhcp_servers.update",
            update_thing!(service.dhcp_servers, DHCPServer),
        )
        .unwrap();

    module
        .register_method::<Result<(), ApiError>, _>(
            "service.dhcp_servers.delete",
            delete_thing!(service.dhcp_servers),
        )
        .unwrap();

    module
        .register_method("service.dns_servers.get", get_thing!(service.dns_servers))
        .unwrap();

    module
        .register_method::<Result<Vec<DNSServer>, ApiError>, _>(
            "service.dns_servers.list",
            list_things!(service.dns_servers),
        )
        .unwrap();

    module
        .register_method::<Result<String, ApiError>, _>(
            "service.dns_servers.create",
            create_thing!(service.dns_servers, DNSServer),
        )
        .unwrap();

    module
        .register_method::<Result<String, ApiError>, _>(
            "service.dns_servers.update",
            update_thing!(service.dns_servers, DNSServer),
        )
        .unwrap();

    module
        .register_method::<Result<(), ApiError>, _>(
            "service.dns_servers.delete",
            delete_thing!(service.dns_servers),
        )
        .unwrap();

    module
        .register_method("service.ntp_servers.get", get_thing!(service.ntp_servers))
        .unwrap();

    module
        .register_method::<Result<Vec<NTPServer>, ApiError>, _>(
            "service.ntp_servers.list",
            list_things!(service.ntp_servers),
        )
        .unwrap();

    module
        .register_method::<Result<String, ApiError>, _>(
            "service.ntp_servers.create",
            create_thing!(service.ntp_servers, NTPServer),
        )
        .unwrap();

    module
        .register_method::<Result<String, ApiError>, _>(
            "service.ntp_servers.update",
            update_thing!(service.ntp_servers, NTPServer),
        )
        .unwrap();

    module
        .register_method::<Result<(), ApiError>, _>(
            "service.ntp_servers.delete",
            delete_thing!(service.ntp_servers),
        )
        .unwrap();
}
