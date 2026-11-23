//! Guest check functions shared between the scenario binary (`main.rs`)
//! and the integration tests (`tests/`) — code that both sides need
//! must live in the lib, so it lands here. What only one side needs
//! lives next to that caller instead: the power-off step is in
//! `main.rs`, the post-reboot persistence assertion is in
//! `tests/apply.rs`.
//!
//! Every check assumes it starts from the `healthy` checkpoint
//! (`Ctx::restore`), so checks are order-independent: each one sees the
//! pristine, fully booted image.
//!
//! **Fixture pattern for checks that mutate the guest** (the
//! apply/reboot/power-loss family). Three steps, all of them:
//!
//! 1. *Pre-state baseline* — assert the fixture is **absent** before it
//!    is created, or the post-state assertion can pass vacuously
//!    (`apply_rule` does this for the E2E rule: absent from the config
//!    and from nftables before the apply);
//! 2. *Persistence confirmation* — assert the fixture reached the
//!    persistence medium **before** the disruptive event (`rule_on_disk`
//!    confirms the applied rule is in `/var/lib/nfsense/config.json`
//!    before the reboot/reset);
//! 3. *Post-state* — assert the fixture is back after the event
//!    (`rule_persists` in `tests/apply.rs`).
//!
//! New persistence-flavoured checks must keep all three: a test that
//! cannot fail proves nothing.

use std::time::{Duration, Instant};

use serde_json::{json, Value};

use crate::api::ApiClient;
use crate::qga::Qga;
use crate::timeouts::{API, BOOT_DRAIN, UNIT};

pub fn nfsense_installed(qga: &mut Qga) -> Result<(), String> {
    let (exit, _, err) = qga.exec(
        "/usr/bin/test",
        &["-x", "/usr/sbin/nfsense"],
        Duration::from_secs(10),
    )?;
    if exit == 0 {
        Ok(())
    } else {
        Err(format!(
            "nfsense binary not found in guest (test exit {exit}: {err})"
        ))
    }
}

/// Poll `systemctl is-active {unit}` until the unit is active.
/// Fresh boots race: the guest agent can come up while nfsense.service is
/// still activating, so a single check is unreliable.
pub fn wait_unit_active(qga: &mut Qga, unit: &str) -> Result<(), String> {
    let deadline = Instant::now() + UNIT;
    let mut last: String;
    loop {
        match qga.exec(
            "/usr/bin/systemctl",
            &["is-active", unit],
            Duration::from_secs(15),
        ) {
            Ok((_, out, _)) if out.trim() == "active" => return Ok(()),
            Ok((exit, out, err)) => last = format!("is-active exit {exit}: {out}{err}"),
            Err(e) => last = e,
        }
        if Instant::now() > deadline {
            let status = qga
                .exec(
                    "/usr/bin/systemctl",
                    &["status", unit, "--no-pager"],
                    Duration::from_secs(15),
                )
                .ok()
                .map(|(_, out, _)| out)
                .unwrap_or_default();
            return Err(format!("unit {unit} not active: {last}\n{status}"));
        }
        std::thread::sleep(Duration::from_secs(2));
    }
}

pub fn nfsense_service(qga: &mut Qga) -> Result<(), String> {
    wait_unit_active(qga, "nfsense")
}

pub fn api_health(qga: &mut Qga) -> Result<(), String> {
    // There is no `ping` JSON-RPC method; use a read-only config call as
    // the health probe.
    let cfg = ApiClient::call(qga, "config.pending.get", json!({}))?;
    if !cfg.is_object() {
        return Err("config.pending.get returned no object".into());
    }
    Ok(())
}

pub fn default_config(qga: &mut Qga) -> Result<(), String> {
    let cfg = ApiClient::call(qga, "config.pending.get", json!({}))?;
    let version = cfg
        .get("config_version")
        .and_then(Value::as_u64)
        .ok_or("missing config_version")?;
    let inbound = cfg
        .get("firewall")
        .and_then(|f| f.get("inbound_rules"))
        .and_then(Value::as_array)
        .ok_or("firewall.inbound_rules is not an array")?;
    if inbound.is_empty() {
        return Err("default config has no inbound rules".into());
    }
    eprintln!("e2e: config v{version}, {} inbound rules", inbound.len());
    Ok(())
}

pub fn nftables_table(qga: &mut Qga) -> Result<(), String> {
    let out = qga.exec_ok(
        "/usr/sbin/nft",
        &["list", "table", "inet", "nfsense_inet"],
        Duration::from_secs(15),
    )?;
    if out.contains("nfsense_inet") {
        for line in out.lines().take(4) {
            eprintln!("e2e:   > {line}");
        }
        Ok(())
    } else {
        Err(format!("nftables table not found:\n{out}"))
    }
}

pub fn nftables_rules(qga: &mut Qga) -> Result<(), String> {
    // Assert *rule content*, not just a non-empty dump: the product
    // renders every rule with a `comment` and named counters, while a
    // skeleton table (chains + policies, zero rules) would still
    // satisfy a mere "non-empty" check.
    let out = qga.exec_ok(
        "/usr/sbin/nft",
        &["list", "table", "inet", "nfsense_inet"],
        Duration::from_secs(15),
    )?;
    let comments = out.lines().filter(|l| l.contains("comment")).count();
    let counters = out.lines().filter(|l| l.contains("counter")).count();
    if comments == 0 || counters == 0 {
        return Err(format!(
            "nftables table has no rules ({comments} commented lines, {counters} counter lines):\n{out}"
        ));
    }
    if !out.contains("policy drop") {
        return Err(format!(
            "inbound/forward chains missing their drop policy:\n{out}"
        ));
    }
    eprintln!("e2e: nftables table has {comments} commented rule lines, {counters} counter lines");
    Ok(())
}

/// The distinctive rule the apply checks push through the product's
/// set (server-side validate + diff) -> apply -> nftables pipeline.
pub fn e2e_rule() -> Value {
    json!({
        "name": "E2E Test Rule",
        "uuid": E2E_RULE_UUID,
        "services": [],
        "source_addresses": [],
        "negate_source": false,
        "destination_addresses": [],
        "negate_destination": false,
        "comment": "e2e apply test",
        "counter": true,
        "log": true,
        "verdict": "accept"
    })
}

/// Shared with `tests/apply.rs`: the post-reboot persistence assertion
/// matches the rule on this uuid.
pub const E2E_RULE_UUID: &str = "e2e00000-0000-4000-8000-000000000001";

/// Add the E2E rule to the pending config, apply it, and verify it
/// landed: in the applied config (after `apply`, the pending config
/// mirrors the current one — the API exposes no `config.current.get`)
/// and in nftables.
///
/// Fixture pattern, step 1 (pre-state baseline): before touching
/// anything this asserts the E2E rule is *absent* from both the config
/// and nftables. The checkpoint is factory state, so absence is
/// expected; if the rule ever leaked in (a checkpoint saved dirty, or
/// stale nftables content), the post-apply assertions would pass
/// vacuously and the persistence tests would prove nothing. Every
/// apply-family test gets this check for free through this function.
pub fn apply_rule(qga: &mut Qga) -> Result<(), String> {
    // Apply restarts kea and friends; their jobs are ordered behind the
    // boot transaction, so wait for the boot job queue to drain first
    // (observe-only — the guest is not modified).
    wait_boot_jobs_drained(qga)?;

    let mut cfg = ApiClient::call(qga, "config.pending.get", json!({}))?;
    let rules = cfg["firewall"]["inbound_rules"]
        .as_array()
        .ok_or("config.pending.get: firewall.inbound_rules not an array")?;
    if rules
        .iter()
        .any(|r| r.get("uuid").and_then(Value::as_str) == Some(E2E_RULE_UUID))
    {
        return Err(
            "E2E rule already present in the pre-apply config (checkpoint not factory-fresh?)"
                .into(),
        );
    }
    // Absent from nftables too — a missing table is fine (the
    // `nftables_table` check owns its existence), content is not.
    let (pre_exit, pre_out, _) = qga.exec(
        "/usr/sbin/nft",
        &["list", "table", "inet", "nfsense_inet"],
        Duration::from_secs(15),
    )?;
    if pre_exit == 0 && pre_out.contains("E2E Test Rule") {
        return Err("E2E rule already present in nftables before apply (stale state?)".into());
    }

    cfg["firewall"]["inbound_rules"]
        .as_array_mut()
        .ok_or("config.pending.get: firewall.inbound_rules not an array")?
        .push(e2e_rule());

    ApiClient::call(qga, "config.pending.set", json!({ "config": cfg }))?;
    ApiClient::call(qga, "config.pending.apply", json!({}))?;

    // The applied config must carry the rule by uuid.
    let applied = ApiClient::call(qga, "config.pending.get", json!({}))?;
    let rules = applied["firewall"]["inbound_rules"]
        .as_array()
        .ok_or("config.pending.get: firewall.inbound_rules not an array")?;
    if !rules
        .iter()
        .any(|r| r.get("uuid").and_then(Value::as_str) == Some(E2E_RULE_UUID))
    {
        return Err("applied config does not contain the E2E rule (by uuid)".into());
    }

    let out = qga.exec_ok(
        "/usr/sbin/nft",
        &["list", "table", "inet", "nfsense_inet"],
        Duration::from_secs(15),
    )?;
    if out.contains("E2E Test Rule") {
        Ok(())
    } else {
        Err(format!("applied config not reflected in nftables:\n{out}"))
    }
}

/// Assert the applied E2E rule is *on disk* in the /var config file.
///
/// Fixture pattern, step 2: call this **before** the reboot/reset (see
/// `tests/apply.rs`). The post-reset assertions ("the rule survived")
/// only mean something if the rule was in
/// `/var/lib/nfsense/config.json` at the moment of the power event —
/// this is that confirmation. `grep -F` on the fixed uuid string:
/// exit 0 (a match) is success, any other outcome is an error.
pub fn rule_on_disk(qga: &mut Qga) -> Result<(), String> {
    qga.exec_ok(
        "grep",
        &["-F", E2E_RULE_UUID, "/var/lib/nfsense/config.json"],
        Duration::from_secs(15),
    )
    .map_err(|e| format!("E2E rule not found in /var/lib/nfsense/config.json: {e}"))?;
    eprintln!("e2e: E2E rule confirmed on /var (config.json) before the power event");
    Ok(())
}

/// Wait until the boot transaction has drained: no jobs are left running.
///
/// This is the image-agnostic form of "the boot is no longer holding
/// anything down": in an unpatched image without an uplink the queue
/// drains when `systemd-networkd-wait-online` (~120 s) and
/// `chrony-wait.service` (~180 s) time out; with the boot-time fixes from
/// `docs/image-service-recommendations.md` it drains at boot. (Gating on
/// `is-active time-sync.target` / `network-online.target` would not work
/// once those targets are no longer pulled into the boot closure: a
/// passive target that is never started stays `inactive` forever, even
/// with nothing pending.)
///
/// These are real properties of the image under test, so the harness does
/// not modify the guest to speed them up (that would falsify the
/// result); it budgets for them instead. kea's restart job (queued by
/// apply) is ordered behind the same boot work, so a drained queue means
/// apply will not stall in the job queue.
pub fn wait_boot_jobs_drained(qga: &mut Qga) -> Result<(), String> {
    const NO_JOBS: &str = "No jobs running.";
    let deadline = Instant::now() + BOOT_DRAIN;
    let start = Instant::now();
    let mut empty_streak = 0u32;
    let mut last = String::from("not attempted");
    let mut last_print = Instant::now();
    loop {
        let out = qga
            .exec_ok(
                "/usr/bin/systemctl",
                &["list-jobs", "--no-pager"],
                Duration::from_secs(15),
            )
            .map_err(|e| format!("list-jobs failed: {e}"))?;
        if out.trim() == NO_JOBS {
            empty_streak += 1;
            // Three consecutive empty polls: tolerate late-queued jobs.
            if empty_streak >= 3 {
                eprintln!(
                    "e2e: boot job queue drained after {}s",
                    start.elapsed().as_secs()
                );
                return Ok(());
            }
        } else {
            empty_streak = 0;
            last = out.lines().take(5).collect::<Vec<_>>().join(" | ");
        }
        if Instant::now() > deadline {
            return Err(format!(
                "boot job queue not drained within {BOOT_DRAIN:?}:\n{out}"
            ));
        }
        if last_print.elapsed() >= Duration::from_secs(10) {
            eprintln!("e2e: waiting for boot to settle... ({last})");
            last_print = Instant::now();
        }
        std::thread::sleep(Duration::from_secs(2));
    }
}

/// Wait for the API (in-guest loopback) to accept a session login.
pub fn api_ready(qga: &mut Qga, mut alive: impl FnMut() -> bool) -> Result<(), String> {
    ApiClient::wait_ready(qga, Instant::now() + API, &mut alive)
}
