use super::ApiError;
use crate::{
    definitions::firewall::{DestinationNATRule, ForwardRule, SourceNATRule},
    delete_vec_thing, get_vec_thing, list_things,
    state::RpcState,
};
use jsonrpsee::RpcModule;

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_method(
            "firewall.forward_rules.get",
            get_vec_thing!(firewall.forward_rules),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<ForwardRule>, ApiError>, _>(
            "firewall.forward_rules.list",
            list_things!(firewall.forward_rules),
        )
        .unwrap();

    module
        .register_method(
            "firewall.forward_rules.delete",
            delete_vec_thing!(firewall.forward_rules),
        )
        .unwrap();

    module
        .register_method(
            "firewall.destination_nat_rules.get",
            get_vec_thing!(firewall.destination_nat_rules),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<DestinationNATRule>, ApiError>, _>(
            "firewall.destination_nat_rules.list",
            list_things!(firewall.destination_nat_rules),
        )
        .unwrap();

    module
        .register_method(
            "firewall.destination_nat_rules.delete",
            delete_vec_thing!(firewall.destination_nat_rules),
        )
        .unwrap();

    module
        .register_method(
            "firewall.source_nat_rules.get",
            get_vec_thing!(firewall.source_nat_rules),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<SourceNATRule>, ApiError>, _>(
            "firewall.source_nat_rules.list",
            list_things!(firewall.source_nat_rules),
        )
        .unwrap();

    module
        .register_method(
            "firewall.source_nat_rules.delete",
            delete_vec_thing!(firewall.source_nat_rules),
        )
        .unwrap();
}
