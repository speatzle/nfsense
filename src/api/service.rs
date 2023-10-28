use std::collections::HashMap;

use jsonrpsee::types::Params;

use crate::{
    definitions::service::{DHCPServer, DNSServer, NTPServer},
    state::RpcState,
};

use super::ApiError;

pub fn get_dhcp_servers(_: Params, state: &RpcState) -> Result<Vec<DHCPServer>, ApiError> {
    Ok(state
        .config_manager
        .get_pending_config()
        .service
        .dhcp_servers)
}

pub fn get_dns_servers(_: Params, state: &RpcState) -> Result<Vec<DNSServer>, ApiError> {
    Ok(state
        .config_manager
        .get_pending_config()
        .service
        .dns_servers)
}

pub fn get_ntp_servers(_: Params, state: &RpcState) -> Result<Vec<NTPServer>, ApiError> {
    Ok(state
        .config_manager
        .get_pending_config()
        .service
        .ntp_servers)
}
