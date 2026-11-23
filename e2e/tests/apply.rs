//! Apply group: set -> validate -> diff -> nftables, plus reboot and
//! power-loss persistence checks.
//!
//! Every persistence test in this file follows the three-step fixture
//! pattern defined in `checks.rs` (pre-state baseline → rule_on_disk →
//! post-state). New persistence checks must keep all three steps: a
//! test that cannot fail proves nothing.

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

/// Apply a rule, reboot the guest for real, and verify the rule
/// *survives*.
///
/// The reboot goes through the production path: an in-guest
/// `systemctl reboot` (`Ctx::reboot`) — PID1's full shutdown (clean
/// unmount of the persistent `/var`, service stops) and a full
/// OVMF + systemd boot. The guest has a TPM 2.0 (host swtpm via
/// QEMU's `emulator` TPM backend), so in-guest repart completes and
/// the config in `/var/lib/nfsense` survives: the service re-derives
/// nftables from it at boot. Before the reboot, `rule_on_disk`
/// confirms the rule actually reached `/var` (fixture pattern step 2).
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

/// Apply a rule, simulate a power loss (QMP `system_reset` — a sudden
/// cold reset with no unmount; `Ctx::hard_reset` syncs first so the
/// applied config reaches disk), and verify the rule still *survives*.
/// `rule_on_disk` confirms the rule was in `/var/lib/nfsense/config.json`
/// before the power cut (fixture pattern step 2).
///
/// Crash-resilience complement to `reboot_after_apply`: if `/var`
/// survives a power cut, it survives every reboot variant.
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

/// After a reboot the applied E2E rule must be *present*: `/var` is the
/// repart-created ext4 partition that persists across boots (the guest
/// has a TPM 2.0 from host swtpm, so in-guest repart completes), so
/// `nfsense-init.service` takes the not-first-boot path and the service
/// re-derives nftables from the persisted config in
/// `/var/lib/nfsense`.
///
/// Used by both reboot variants: `reboot_after_apply` (graceful,
/// in-guest `systemctl reboot`) and `power_loss_after_apply` (hard
/// QMP `system_reset`).
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

    // The documented mechanism, asserted directly: `/var` must be the
    // repart-created ext4 partition (not the root-tmpfs fallback) and
    // the product config must live on it.
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
