use super::ApiError;
use crate::{
    create_thing,
    definitions::firewall::{DestinationNATRule, ForwardRule, SourceNATRule},
    delete_thing_by_index, get_thing_by_index, list_things,
    state::RpcState,
    update_thing_by_index,
};
use jsonrpsee::{Extensions, RpcModule};

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_method(
            "firewall.forward_rules.get",
            get_thing_by_index!(firewall.forward_rules),
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
            "firewall.forward_rules.create",
            create_thing!(firewall.forward_rules, ForwardRule),
        )
        .unwrap();

    module
        .register_method(
            "firewall.forward_rules.update",
            update_thing_by_index!(firewall.forward_rules, ForwardRule),
        )
        .unwrap();

    module
        .register_method(
            "firewall.forward_rules.delete",
            delete_thing_by_index!(firewall.forward_rules),
        )
        .unwrap();

    module
        .register_method(
            "firewall.destination_nat_rules.get",
            get_thing_by_index!(firewall.destination_nat_rules),
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
            "firewall.destination_nat_rules.create",
            create_thing!(firewall.destination_nat_rules, DestinationNATRule),
        )
        .unwrap();

    module
        .register_method(
            "firewall.destination_nat_rules.update",
            update_thing_by_index!(firewall.destination_nat_rules, DestinationNATRule),
        )
        .unwrap();

    module
        .register_method(
            "firewall.destination_nat_rules.delete",
            delete_thing_by_index!(firewall.destination_nat_rules),
        )
        .unwrap();

    module
        .register_method(
            "firewall.source_nat_rules.get",
            get_thing_by_index!(firewall.source_nat_rules),
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
            "firewall.source_nat_rules.create",
            create_thing!(firewall.source_nat_rules, SourceNATRule),
        )
        .unwrap();

    module
        .register_method(
            "firewall.source_nat_rules.update",
            update_thing_by_index!(firewall.source_nat_rules, SourceNATRule),
        )
        .unwrap();

    module
        .register_method(
            "firewall.source_nat_rules.delete",
            delete_thing_by_index!(firewall.source_nat_rules),
        )
        .unwrap();
}
