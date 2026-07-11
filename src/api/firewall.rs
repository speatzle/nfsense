use super::ApiError;
use crate::{
    definitions::firewall::{DestinationNATRule, ForwardRule, InboundRule, SourceNATRule},
    delete_thing, get_thing, list_things, move_thing,
    state::RpcState,
};
use jsonrpsee::{Extensions, RpcModule};
use serde::Deserialize;
use uuid::Uuid;

use crate::commit_and_changelog;
use time::OffsetDateTime;

macro_rules! create_firewall_thing {
    ($( $sub_system:ident ).+, $typ:ty) => {
        |params, state, extensions: &jsonrpsee::Extensions| {
            let mut t: $typ = params.parse().map_err(ApiError::ParameterDeserialize)?;
            if t.uuid.is_nil() {
                t.uuid = Uuid::new_v4();
            }
            let id = structdb_core::Keyed::get_key(&t);
            let mut cm = state.config_manager.clone();
            let mut tx = cm.start_transaction();
            tx.data_mut().$($sub_system).+.push(t);
            commit_and_changelog!(cm, tx, extensions)?;
            Ok(id)
        }
    };
}

macro_rules! update_firewall_thing {
    ($( $sub_system:ident ).+, $typ:ty) => {
        |params, state, extensions: &jsonrpsee::Extensions| {
            use serde::Serialize;
            use tracing::info;

            #[derive(Deserialize, Serialize)]
            struct UpdateThing {
                id: String,
                thing: $typ,
            }

            let t: UpdateThing = params.parse().map_err(ApiError::ParameterDeserialize)?;
            let mut cm = state.config_manager.clone();
            let mut tx = cm.start_transaction();

            let index = tx.data_mut()
                .$($sub_system).+
                .iter()
                .position(|e| structdb_core::Keyed::get_key(e) == t.id);

            match index {
                Some(i) => {
                    let old_key = structdb_core::Keyed::get_key(&tx.data_mut().$($sub_system).+[i]);
                    let new_key = structdb_core::Keyed::get_key(&t.thing);
                    info!("check if key are equal {} == {}", old_key, new_key);
                    if old_key != new_key {
                        structdb_core::RenameRefs::rename_refs::<$typ>(
                            tx.data_mut(),
                            &old_key,
                            &new_key,
                        );
                    }
                    let mut thing = t.thing;
                    if thing.uuid.is_nil() {
                        thing.uuid = tx.data_mut().$($sub_system).+[i].uuid;
                    }
                    tx.data_mut().$($sub_system).+[i] = thing;
                    commit_and_changelog!(cm, tx, extensions)?;
                    Ok(new_key)
                }
                None => {
                    tx.revert();
                    Err(ApiError::NotFound)
                }
            }
        }
    };
}

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_method(
            "firewall.forward_rules.get",
            get_thing!(firewall.forward_rules),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<ForwardRule>, ApiError>, _>(
            "firewall.forward_rules.list",
            list_things!(firewall.forward_rules),
        )
        .unwrap();

    module
        .register_method::<Result<String, ApiError>, _>(
            "firewall.forward_rules.create",
            create_firewall_thing!(firewall.forward_rules, ForwardRule),
        )
        .unwrap();

    module
        .register_method::<Result<String, ApiError>, _>(
            "firewall.forward_rules.update",
            update_firewall_thing!(firewall.forward_rules, ForwardRule),
        )
        .unwrap();

    module
        .register_method::<Result<(), ApiError>, _>(
            "firewall.forward_rules.delete",
            delete_thing!(firewall.forward_rules),
        )
        .unwrap();

    module
        .register_method::<Result<(), ApiError>, _>(
            "firewall.forward_rules.move",
            move_thing!(firewall.forward_rules),
        )
        .unwrap();

    module
        .register_method(
            "firewall.destination_nat_rules.get",
            get_thing!(firewall.destination_nat_rules),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<DestinationNATRule>, ApiError>, _>(
            "firewall.destination_nat_rules.list",
            list_things!(firewall.destination_nat_rules),
        )
        .unwrap();

    module
        .register_method::<Result<String, ApiError>, _>(
            "firewall.destination_nat_rules.create",
            create_firewall_thing!(firewall.destination_nat_rules, DestinationNATRule),
        )
        .unwrap();

    module
        .register_method::<Result<String, ApiError>, _>(
            "firewall.destination_nat_rules.update",
            update_firewall_thing!(firewall.destination_nat_rules, DestinationNATRule),
        )
        .unwrap();

    module
        .register_method::<Result<(), ApiError>, _>(
            "firewall.destination_nat_rules.delete",
            delete_thing!(firewall.destination_nat_rules),
        )
        .unwrap();

    module
        .register_method::<Result<(), ApiError>, _>(
            "firewall.destination_nat_rules.move",
            move_thing!(firewall.destination_nat_rules),
        )
        .unwrap();

    module
        .register_method(
            "firewall.source_nat_rules.get",
            get_thing!(firewall.source_nat_rules),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<SourceNATRule>, ApiError>, _>(
            "firewall.source_nat_rules.list",
            list_things!(firewall.source_nat_rules),
        )
        .unwrap();

    module
        .register_method::<Result<String, ApiError>, _>(
            "firewall.source_nat_rules.create",
            create_firewall_thing!(firewall.source_nat_rules, SourceNATRule),
        )
        .unwrap();

    module
        .register_method::<Result<String, ApiError>, _>(
            "firewall.source_nat_rules.update",
            update_firewall_thing!(firewall.source_nat_rules, SourceNATRule),
        )
        .unwrap();

    module
        .register_method::<Result<(), ApiError>, _>(
            "firewall.source_nat_rules.delete",
            delete_thing!(firewall.source_nat_rules),
        )
        .unwrap();

    module
        .register_method::<Result<(), ApiError>, _>(
            "firewall.source_nat_rules.move",
            move_thing!(firewall.source_nat_rules),
        )
        .unwrap();

    module
        .register_method(
            "firewall.inbound_rules.get",
            get_thing!(firewall.inbound_rules),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<InboundRule>, ApiError>, _>(
            "firewall.inbound_rules.list",
            list_things!(firewall.inbound_rules),
        )
        .unwrap();

    module
        .register_method::<Result<String, ApiError>, _>(
            "firewall.inbound_rules.create",
            create_firewall_thing!(firewall.inbound_rules, InboundRule),
        )
        .unwrap();

    module
        .register_method::<Result<String, ApiError>, _>(
            "firewall.inbound_rules.update",
            update_firewall_thing!(firewall.inbound_rules, InboundRule),
        )
        .unwrap();

    module
        .register_method::<Result<(), ApiError>, _>(
            "firewall.inbound_rules.delete",
            delete_thing!(firewall.inbound_rules),
        )
        .unwrap();

    module
        .register_method::<Result<(), ApiError>, _>(
            "firewall.inbound_rules.move",
            move_thing!(firewall.inbound_rules),
        )
        .unwrap();
}
