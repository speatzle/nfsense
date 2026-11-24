mod common;

use common::*;
use nfsense_e2e::checks;
use serde_json::json;

#[test]
fn f2_2_delete_takes_id() {
    with_ctx(|ctx| {
        let qga = ctx.agent()?;
        call(
            qga,
            "object.addresses.create",
            host("e2e-del", "192.0.2.10"),
        )?;
        rejected(qga, "object.addresses.delete", json!({ "name": "e2e-del" }))?;
        call(qga, "object.addresses.delete", json!({ "id": "e2e-del" }))?;
        Ok(())
    });
}

// A panic in the apply path drops the connection while systemd still shows
// the service `active`, so check that the API answers and nothing panicked.
#[test]
fn f3_4_failed_apply_keeps_api_alive() {
    with_ctx(|ctx| {
        let qga = ctx.agent()?;
        checks::wait_boot_jobs_drained(qga)?;
        call(qga, "object.services.create", tcp_port("e2e-http", 80))?;
        call(
            qga,
            "object.services.create",
            tcp_range("e2e-range", 1000, 2000),
        )?;
        call(
            qga,
            "object.addresses.create",
            host("e2e-dnat-target", "192.0.2.20"),
        )?;
        call(
            qga,
            "firewall.destination_nat_rules.create",
            json!({
                "name": "e2e-dnat", "uuid": uuid(4), "services": ["e2e-http"],
                "source_addresses": [], "negate_source": false,
                "destination_addresses": [], "negate_destination": false,
                "comment": "", "automatic_forward_rule": false,
                "counter": false, "log": false,
                "dnat_address": "e2e-dnat-target", "dnat_service": "e2e-range"
            }),
        )?;
        rejected(qga, "config.pending.apply", json!({}))
            .map_err(|e| format!("apply of an unrenderable DNAT rule: {e}"))?;
        call(qga, "config.pending.log", json!({}))
            .map_err(|e| format!("API dead after the failed apply: {e}"))?;
        let (_, out) = sh(
            qga,
            "journalctl -b -u nfsense --no-pager -o cat | grep -c panicked",
        )?;
        ensure(out.trim() == "0", "the daemon logged a panic", &out)
    });
}

#[test]
fn f3_7_discard_survives_restart() {
    with_ctx(|ctx| {
        let qga = ctx.agent()?;
        call(
            qga,
            "object.addresses.create",
            host("e2e-discard", "192.0.2.99"),
        )?;
        call(qga, "config.pending.discard", json!({}))?;
        sh(qga, "systemctl restart nfsense")?;
        checks::wait_unit_active(qga, "nfsense")?;
        checks::api_ready(qga, || true)?;
        let log = call(qga, "config.pending.log", json!({}))?;
        ensure(
            !log.to_string().contains("e2e-discard"),
            "discarded change is back after a restart",
            &log.to_string(),
        )
    });
}

#[test]
fn f3_13_stop_is_graceful() {
    with_ctx(|ctx| {
        let (_, out) = sh(
            ctx.agent()?,
            "systemctl stop nfsense; systemctl show -p Result nfsense; \
             journalctl -b -u nfsense --no-pager -o cat | grep -i 'shutting down\\|shutdown complete'",
        )?;
        ensure(
            out.contains("Result=success") && out.contains("shutting down"),
            "nfsense did not shut down gracefully",
            &out,
        )
    });
}
