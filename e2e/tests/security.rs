mod common;

use common::*;
use nfsense_e2e::checks;
use serde_json::json;

#[test]
fn f3_11_service_control_allowlist() {
    with_ctx(|ctx| {
        let qga = ctx.agent()?;
        rejected(qga, "system.services.restart", json!("getty@tty1.service"))?;
        call(qga, "system.services.restart", json!("chronyd.service"))?;
        Ok(())
    });
}

#[test]
fn f3_9_wireguard_key_validated() {
    with_ctx(|ctx| {
        let qga = ctx.agent()?;
        rejected(
            qga,
            "vpn.wireguard.peers.create",
            json!({
                "name": "e2e-bad", "public_key": "not-a-key\n[Match]",
                "preshared_key": null, "allowed_ips": [], "endpoint": "none",
                "persistent_keepalive": null, "comment": ""
            }),
        )?;
        Ok(())
    });
}

#[test]
fn f3_15_wireguard_ipv6_endpoint_bracketed() {
    with_ctx(|ctx| {
        let qga = ctx.agent()?;
        checks::wait_boot_jobs_drained(qga)?;
        call(
            qga,
            "object.addresses.create",
            host("e2e-v6peer", "2001:db8::1"),
        )?;
        call(
            qga,
            "vpn.wireguard.peers.create",
            json!({
                "name": "e2e-peer", "public_key": WG_KEY_B, "preshared_key": null,
                "allowed_ips": [],
                "endpoint": { "specify": { "address": "e2e-v6peer", "port": 51820 } },
                "persistent_keepalive": 25, "comment": ""
            }),
        )?;
        call(
            qga,
            "vpn.wireguard.interfaces.create",
            json!({
                "name": "wg0", "public_key": "", "private_key": WG_KEY_A,
                "listen_port": 51820, "peers": ["e2e-peer"], "comment": ""
            }),
        )?;
        call(qga, "config.pending.apply", json!({}))?;
        let (_, out) = sh(
            qga,
            "f=$(ls /run/systemd/network/*wireguard*wg0*.netdev); \
             grep -i '^Endpoint' \"$f\"; sleep 2; \
             ip -d link show wg0 | head -3",
        )?;
        ensure(
            out.contains("Endpoint=[2001:db8::1]:51820"),
            "IPv6 endpoint not bracketed",
            &out,
        )?;
        ensure(out.contains("wireguard"), "wg0 was not created", &out)
    });
}

fn hw_iface(name: &str, device: &str) -> serde_json::Value {
    serde_json::json!({
        "name": name, "alias": "", "comment": "",
        "interface_type": { "hardware": { "device": device } },
        "addressing_mode": { "static": { "address": "192.168.10.1/24" } },
        "virtual_router": null
    })
}

#[test]
fn f3_9_device_names_validated() {
    with_ctx(|ctx| {
        let qga = ctx.agent()?;
        for device in ["eth0:1", ".", "..", "eth0\n[Match]"] {
            rejected(qga, "network.interfaces.create", hw_iface("bad", device))?;
        }
        call(
            qga,
            "network.interfaces.create",
            hw_iface("good", "eth0.10"),
        )?;
        Ok(())
    });
}
