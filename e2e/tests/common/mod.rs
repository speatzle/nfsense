#![allow(dead_code)]

use std::sync::{Mutex, OnceLock};

use nfsense_e2e::ctx::{Ctx, InitError};
use nfsense_e2e::image::default_image_dir;

// Serialised because the QGA chardev accepts one host connection at a time.
// A failed init is cached so each test does not re-wait on the run lock.
static CTX: OnceLock<Mutex<Option<Ctx>>> = OnceLock::new();
static INIT_FAIL: OnceLock<String> = OnceLock::new();

fn image_dir() -> std::path::PathBuf {
    std::env::var_os("E2E_IMAGE_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(default_image_dir)
}

// Shutdown cannot rely on a static's Drop: a failing libtest binary exits via
// `process::exit`, which skips static destructors.
struct ShutdownGuard<'a> {
    ctx: &'a mut Ctx,
}

impl Drop for ShutdownGuard<'_> {
    fn drop(&mut self) {
        eprintln!("e2e: shutting VM down after test");
        self.ctx.shutdown();
    }
}

pub fn with_ctx(f: impl FnOnce(&mut Ctx) -> Result<(), String>) {
    let m = CTX.get_or_init(|| Mutex::new(None));
    let mut g = m.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let ctx = match g.as_mut() {
        Some(c) => c,
        None => {
            if let Some(fail) = INIT_FAIL.get() {
                panic!("e2e FAIL: {fail}");
            }
            match Ctx::try_new(Some(&image_dir())) {
                Ok(c) => g.insert(c),
                Err(InitError::Skip(r)) => {
                    eprintln!("e2e SKIP: {r}");
                    return;
                }
                Err(InitError::Fail(r)) => {
                    let _ = INIT_FAIL.set(r.clone());
                    panic!("e2e FAIL: {r}");
                }
            }
        }
    };

    if let Err(e) = ctx.restore() {
        ctx.shutdown();
        panic!("e2e: restore to checkpoint failed: {e}");
    }

    let mut guard = ShutdownGuard { ctx };
    let ctx = &mut guard.ctx;
    if let Err(e) = f(ctx) {
        panic!("{e}");
    }
}

use std::time::Duration;

use nfsense_e2e::api::ApiClient;
use nfsense_e2e::qga::Qga;
use serde_json::{json, Value};

pub fn sh(qga: &mut Qga, script: &str) -> Result<(i64, String), String> {
    let (exit, out, err) = qga.exec("/bin/sh", &["-c", script], Duration::from_secs(120))?;
    Ok((exit, format!("{out}{err}")))
}

pub fn call(qga: &mut Qga, method: &str, params: Value) -> Result<Value, String> {
    ApiClient::call(qga, method, params)
}

/// Returns the JSON-RPC error; a dropped connection or a success is an `Err`.
pub fn rejected(qga: &mut Qga, method: &str, params: Value) -> Result<Value, String> {
    match ApiClient::call_rpc(qga, method, params)? {
        Ok(v) => Err(format!("{method} was accepted, expected a rejection: {v}")),
        Err(e) => Ok(e),
    }
}

pub fn pending_count(qga: &mut Qga) -> Result<usize, String> {
    Ok(call(qga, "config.pending.log", json!({}))?
        .as_array()
        .map_or(0, Vec::len))
}

pub fn uuid(n: u64) -> String {
    format!("e2e00000-0000-4000-8000-{n:012x}")
}

pub fn host(name: &str, ip: &str) -> Value {
    json!({ "name": name, "comment": "", "address_type": { "host": { "address": ip } } })
}

pub fn addr_group(name: &str, members: &[&str]) -> Value {
    json!({ "name": name, "comment": "", "address_type": { "group": { "members": members } } })
}

pub fn tcp_port(name: &str, port: u16) -> Value {
    json!({ "name": name, "comment": "", "service_type": { "tcp": {
        "source": "any", "destination": { "single": { "port": port } } } } })
}

pub fn tcp_range(name: &str, start: u16, end: u16) -> Value {
    json!({ "name": name, "comment": "", "service_type": { "tcp": {
        "source": "any", "destination": { "range": { "start_port": start, "end_port": end } } } } })
}

pub fn svc_group(name: &str, members: &[&str]) -> Value {
    json!({ "name": name, "comment": "", "service_type": { "group": { "members": members } } })
}

pub fn inbound(n: u64, name: &str, services: &[&str], verdict: &str) -> Value {
    json!({
        "name": name, "uuid": uuid(n), "services": services,
        "source_addresses": [], "negate_source": false,
        "destination_addresses": [], "negate_destination": false,
        "comment": "", "counter": true, "log": false, "verdict": verdict
    })
}

// Test-only keys, not secret.
pub const WG_KEY_A: &str = "YNqHbfBQKaGvzefSSc7VVnSZNNrRqcOeUyBqahn0vF4=";
pub const WG_KEY_B: &str = "kAGhtK0Lc3bS9Am1w5xd2u6Da4bFbVJYb9kc6U3ZSnY=";

pub fn ensure(cond: bool, what: &str, evidence: &str) -> Result<(), String> {
    if cond {
        Ok(())
    } else {
        Err(format!("{what}\n--- evidence ---\n{evidence}"))
    }
}
