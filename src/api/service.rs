use super::ApiError;
use crate::{
    definitions::service::{DHCPServer, DNSServer, NTPServer},
    get_things, get_vec_thing,
    state::RpcState,
};
use jsonrpsee::RpcModule;

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_method(
            "service.get_dhcp_server",
            get_vec_thing!(service.dhcp_servers),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<DHCPServer>, ApiError>, _>(
            "service.get_dhcp_servers",
            get_things!(service.dhcp_servers),
        )
        .unwrap();

    module
        .register_method(
            "service.get_dns_server",
            get_vec_thing!(service.dns_servers),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<DNSServer>, ApiError>, _>(
            "service.get_dns_servers",
            get_things!(service.dns_servers),
        )
        .unwrap();

    module
        .register_method(
            "service.get_ntp_server",
            get_vec_thing!(service.ntp_servers),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<NTPServer>, ApiError>, _>(
            "service.get_ntp_servers",
            get_things!(service.ntp_servers),
        )
        .unwrap();
}
