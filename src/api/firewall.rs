use super::ApiError;
use crate::{
    definitions::firewall::{
        DestinationNATRule, ForwardRule, InboundRule, RuleWithCounts, SourceNATRule,
    },
    delete_thing, get_thing, move_thing,
    state::RpcState,
};
use jsonrpsee::{Extensions, RpcModule};
use nftables::{helper, schema::NfListObject, schema::NfObject};
use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;

use crate::commit_and_changelog;
use time::OffsetDateTime;
use tracing::warn;

fn reset_counter(counter_name: &str) -> Result<(), ApiError> {
    let output = std::process::Command::new("nft")
        .args(["reset", "counter", "inet", "nfsense_inet", counter_name])
        .output()
        .map_err(|e| {
            warn!("Failed to run nft reset counter: {}", e);
            ApiError::CommandError
        })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warn!("nft reset counter failed: {}", stderr);
        return Err(ApiError::CommandError);
    }
    Ok(())
}

fn get_counter_counts() -> HashMap<Uuid, (u64, u64)> {
    match helper::get_current_ruleset_with_args(
        helper::DEFAULT_NFT,
        vec!["list", "counters", "table", "inet", "nfsense_inet"],
    ) {
        Ok(ruleset) => {
            let mut counts = HashMap::new();
            for obj in ruleset.objects.iter() {
                if let NfObject::ListObject(NfListObject::Counter(counter)) = obj {
                    let name = counter.name.to_string();
                    let packets = counter.packets.unwrap_or(0) as u64;
                    let bytes = counter.bytes.unwrap_or(0) as u64;
                    // Strip prefix to get UUID — counter name format is {prefix}{uuid}
                    if let Some(uuid_str) = name
                        .strip_prefix("inbound_")
                        .or_else(|| name.strip_prefix("dnat_"))
                        .or_else(|| name.strip_prefix("snat_"))
                        .or_else(|| name.strip_prefix("fw_"))
                    {
                        if let Ok(uuid) = Uuid::parse_str(uuid_str) {
                            counts.insert(uuid, (packets, bytes));
                        }
                    }
                }
            }
            counts
        }
        Err(e) => {
            warn!("Failed to fetch counter counts: {}", e);
            HashMap::new()
        }
    }
}

fn enrich_forward_rules(
    rules: Vec<ForwardRule>,
    counts_by_uuid: &HashMap<Uuid, (u64, u64)>,
) -> Vec<RuleWithCounts<ForwardRule>> {
    rules
        .into_iter()
        .map(|rule| match counts_by_uuid.get(&rule.uuid) {
            Some(&(p, b)) => RuleWithCounts {
                counter_packets: p as i64,
                counter_bytes: b as i64,
                rule,
            },
            None => RuleWithCounts {
                counter_packets: -1,
                counter_bytes: -1,
                rule,
            },
        })
        .collect()
}

fn enrich_dnat_rules(
    rules: Vec<DestinationNATRule>,
    counts_by_uuid: &HashMap<Uuid, (u64, u64)>,
) -> Vec<RuleWithCounts<DestinationNATRule>> {
    rules
        .into_iter()
        .map(|rule| match counts_by_uuid.get(&rule.uuid) {
            Some(&(p, b)) => RuleWithCounts {
                counter_packets: p as i64,
                counter_bytes: b as i64,
                rule,
            },
            None => RuleWithCounts {
                counter_packets: -1,
                counter_bytes: -1,
                rule,
            },
        })
        .collect()
}

fn enrich_snat_rules(
    rules: Vec<SourceNATRule>,
    counts_by_uuid: &HashMap<Uuid, (u64, u64)>,
) -> Vec<RuleWithCounts<SourceNATRule>> {
    rules
        .into_iter()
        .map(|rule| match counts_by_uuid.get(&rule.uuid) {
            Some(&(p, b)) => RuleWithCounts {
                counter_packets: p as i64,
                counter_bytes: b as i64,
                rule,
            },
            None => RuleWithCounts {
                counter_packets: -1,
                counter_bytes: -1,
                rule,
            },
        })
        .collect()
}

fn enrich_inbound_rules(
    rules: Vec<InboundRule>,
    counts_by_uuid: &HashMap<Uuid, (u64, u64)>,
) -> Vec<RuleWithCounts<InboundRule>> {
    rules
        .into_iter()
        .map(|rule| match counts_by_uuid.get(&rule.uuid) {
            Some(&(p, b)) => RuleWithCounts {
                counter_packets: p as i64,
                counter_bytes: b as i64,
                rule,
            },
            None => RuleWithCounts {
                counter_packets: -1,
                counter_bytes: -1,
                rule,
            },
        })
        .collect()
}

use serde::Serialize;

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

macro_rules! list_firewall_things {
    ($( $sub_system:ident ).+, $prefix:expr, $enrich_fn:expr) => {
        |_, state, _: &Extensions| {
            let rules = state
                .config_manager
                .get_pending_config()
                .$($sub_system).+
                .clone();
            let counts = get_counter_counts();
            Ok($enrich_fn(rules, &counts))
        }
    };
}

macro_rules! reset_firewall_counter {
($( $sub_system:ident ).+, $prefix:expr) => {
    |params, state, _: &Extensions| {
        #[derive(Deserialize)]
        struct ResetParams {
            name: String,
        }

        let p: ResetParams = params.parse().map_err(ApiError::ParameterDeserialize)?;

        let config = state.config_manager.get_pending_config();
        let rule = config
            .$($sub_system).+
            .iter()
            .find(|e| e.name == p.name)
            .ok_or(ApiError::NotFound)?;

        let counter_name = format!("{}{}", $prefix, rule.uuid);
        reset_counter(&counter_name)
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
        .register_method::<Result<Vec<RuleWithCounts<ForwardRule>>, ApiError>, _>(
            "firewall.forward_rules.list",
            list_firewall_things!(firewall.forward_rules, "fw_", enrich_forward_rules),
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
        .register_method::<Result<(), ApiError>, _>(
            "firewall.forward_rules.reset_counter",
            reset_firewall_counter!(firewall.forward_rules, "fw_"),
        )
        .unwrap();

    module
        .register_method(
            "firewall.destination_nat_rules.get",
            get_thing!(firewall.destination_nat_rules),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<RuleWithCounts<DestinationNATRule>>, ApiError>, _>(
            "firewall.destination_nat_rules.list",
            list_firewall_things!(firewall.destination_nat_rules, "dnat_", enrich_dnat_rules),
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
        .register_method::<Result<(), ApiError>, _>(
            "firewall.destination_nat_rules.reset_counter",
            reset_firewall_counter!(firewall.destination_nat_rules, "dnat_"),
        )
        .unwrap();

    module
        .register_method(
            "firewall.source_nat_rules.get",
            get_thing!(firewall.source_nat_rules),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<RuleWithCounts<SourceNATRule>>, ApiError>, _>(
            "firewall.source_nat_rules.list",
            list_firewall_things!(firewall.source_nat_rules, "snat_", enrich_snat_rules),
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
        .register_method::<Result<(), ApiError>, _>(
            "firewall.source_nat_rules.reset_counter",
            reset_firewall_counter!(firewall.source_nat_rules, "snat_"),
        )
        .unwrap();

    module
        .register_method(
            "firewall.inbound_rules.get",
            get_thing!(firewall.inbound_rules),
        )
        .unwrap();

    module
        .register_method::<Result<Vec<RuleWithCounts<InboundRule>>, ApiError>, _>(
            "firewall.inbound_rules.list",
            list_firewall_things!(firewall.inbound_rules, "inbound_", enrich_inbound_rules),
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

    module
        .register_method::<Result<(), ApiError>, _>(
            "firewall.inbound_rules.reset_counter",
            reset_firewall_counter!(firewall.inbound_rules, "inbound_"),
        )
        .unwrap();
}
