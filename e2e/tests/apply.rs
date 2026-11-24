mod common;

use std::time::Duration;

use serde_json::{json, Value};

use common::with_ctx;
use nfsense_e2e::api::ApiClient;
use nfsense_e2e::checks::{self, E2E_RULE_UUID};
use nfsense_e2e::qga::Qga;

#[test]
fn apply_rule() {
    with_ctx(|ctx| {
        let qga = ctx.agent()?;
        checks::apply_rule(qga)
    });
}

#[test]
fn reboot_after_apply() {
    with_ctx(|ctx| {
        let qga = ctx.agent()?;
        checks::apply_rule(qga)?;
        checks::rule_on_disk(qga)?;

        ctx.reboot()?;

        let qga = ctx.agent()?;
        checks::api_health(qga)?;
        checks::nftables_table(qga)?;
        rule_persists(qga)
    });
}

// Complements `reboot_after_apply` with a reset that skips the unmount: if
// `/var` survives a power cut, it survives every reboot variant.
#[test]
fn power_loss_after_apply() {
    with_ctx(|ctx| {
        let qga = ctx.agent()?;
        checks::apply_rule(qga)?;
        checks::rule_on_disk(qga)?;

        ctx.hard_reset()?;

        let qga = ctx.agent()?;
        checks::api_health(qga)?;
        checks::nftables_table(qga)?;
        rule_persists(qga)
    });
}

fn rule_persists(qga: &mut Qga) -> Result<(), String> {
    let cfg = ApiClient::call(qga, "config.pending.get", json!({}))?;
    let inbound = cfg
        .get("firewall")
        .and_then(|f| f.get("inbound_rules"))
        .and_then(Value::as_array)
        .ok_or("config.pending.get: firewall.inbound_rules not an array")?;
    if !inbound
        .iter()
        .any(|r| r.get("uuid").and_then(Value::as_str) == Some(E2E_RULE_UUID))
    {
        return Err(
            "E2E rule missing from config after reboot (expected persistence via /var)".into(),
        );
    }
    let out = qga.exec_ok(
        "/usr/sbin/nft",
        &["list", "table", "inet", "nfsense_inet"],
        Duration::from_secs(15),
    )?;
    if !out.contains("E2E Test Rule") {
        return Err(format!(
            "E2E rule missing from nftables after reboot:\n{out}"
        ));
    }

    let fs = qga.exec_ok("df", &["-T", "/var"], Duration::from_secs(15))?;
    if !fs.lines().any(|l| l.contains("ext4")) {
        return Err(format!(
            "'/var' is not ext4 (expected the repart partition):\n{fs}"
        ));
    }
    qga.exec_ok(
        "/usr/bin/test",
        &["-f", "/var/lib/nfsense/config.json"],
        Duration::from_secs(10),
    )
    .map_err(|e| format!("config.json missing on /var: {e}"))?;
    Ok(())
}
