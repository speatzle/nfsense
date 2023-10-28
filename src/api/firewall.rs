use jsonrpsee::types::Params;

use crate::{
    definitions::firewall::{DestinationNATRule, ForwardRule, SourceNATRule},
    state::RpcState,
};

use super::ApiError;

pub fn get_forward_rules(_: Params, state: &RpcState) -> Result<Vec<ForwardRule>, ApiError> {
    Ok(state
        .config_manager
        .get_pending_config()
        .firewall
        .forward_rules)
}

pub fn get_source_nat_rules(_: Params, state: &RpcState) -> Result<Vec<SourceNATRule>, ApiError> {
    Ok(state
        .config_manager
        .get_pending_config()
        .firewall
        .source_nat_rules)
}

pub fn get_destination_nat_rules(
    _: Params,
    state: &RpcState,
) -> Result<Vec<DestinationNATRule>, ApiError> {
    Ok(state
        .config_manager
        .get_pending_config()
        .firewall
        .destination_nat_rules)
}
