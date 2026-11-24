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

// Fresh boots race: the guest agent can come up while the unit is still
// activating, so a single check is unreliable.
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
            Err(e) if qga.is_broken() => return Err(e),
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
    // Assert rule content, not just a non-empty dump: a skeleton table
    // (chains + policies, zero rules) would pass a "non-empty" check, while
    // the product renders every rule with a `comment` and named counters.
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

pub const E2E_RULE_UUID: &str = "e2e00000-0000-4000-8000-000000000001";

// After apply the pending config mirrors the current one (the API has no
// `config.current.get`).
//
// The rule must be absent before the apply, from the config and from
// nftables: if it ever leaked into the checkpoint, the post-apply and
// persistence assertions would pass vacuously.
pub fn apply_rule(qga: &mut Qga) -> Result<(), String> {
    // Apply restarts kea and friends, whose jobs are ordered behind the boot
    // transaction.
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

// Call before the reboot/reset: the post-reset "rule survived" assertions
// only mean something if the rule was on disk at the power event.
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

// Gating on `is-active time-sync.target` / `network-online.target` would not
// work: a passive target that is never pulled into the boot stays `inactive`
// forever. The slow boot units (wait-online, chrony-wait) are real properties
// of the image under test, so the harness budgets for them instead of
// modifying the guest, which would falsify the result.
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

pub fn api_ready(qga: &mut Qga, mut alive: impl FnMut() -> bool) -> Result<(), String> {
    ApiClient::wait_ready(qga, Instant::now() + API, &mut alive)
}
