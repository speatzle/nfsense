use super::ApiError;
use crate::{
    definitions::firewall::{DestinationNATRule, ForwardRule, SourceNATRule},
    delete_vec_thing, get_things, get_vec_thing,
    state::RpcState,
};
use jsonrpsee::RpcModule;

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_method(
            "firewall.get_forward_rule",
            get_vec_thing!(firewall.forward_rules),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<ForwardRule>, ApiError>, _>(
            "firewall.get_forward_rules",
            get_things!(firewall.forward_rules),
        )
        .unwrap();

    module
        .register_method(
            "firewall.delete_forward_rule",
            delete_vec_thing!(firewall.forward_rules),
        )
        .unwrap();

    module
        .register_method(
            "firewall.get_destination_nat_rule",
            get_vec_thing!(firewall.destination_nat_rules),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<DestinationNATRule>, ApiError>, _>(
            "firewall.get_destination_nat_rules",
            get_things!(firewall.destination_nat_rules),
        )
        .unwrap();

    module
        .register_method(
            "firewall.delete_destination_nat_rule",
            delete_vec_thing!(firewall.destination_nat_rules),
        )
        .unwrap();

    module
        .register_method(
            "firewall.get_source_nat_rule",
            get_vec_thing!(firewall.source_nat_rules),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<SourceNATRule>, ApiError>, _>(
            "firewall.get_source_nat_rules",
            get_things!(firewall.source_nat_rules),
        )
        .unwrap();

    module
        .register_method(
            "firewall.delete_source_nat_rule",
            delete_vec_thing!(firewall.source_nat_rules),
        )
        .unwrap();
}
