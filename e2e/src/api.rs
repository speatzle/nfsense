//! JSON-RPC client for the nfsense web API, called *from inside the
//! guest* over loopback.
//!
//! Every call is a `curl` to `127.0.0.1:4444` executed through QGA. The
//! JSON body is passed as a positional shell parameter (`$1`) so it is
//! never shell-quoted. The response is written to a guest-side file;
//! stdout is "HTTP code\n<body>". The session cookie lives in a
//! guest-side cookie jar.

use std::time::{Duration, Instant};

use serde_json::{json, Value};

use crate::qga::Qga;
use crate::timeouts::API_CALL;

const LOGIN_SCRIPT: &str = r#"code=$(curl -sk -o /tmp/e2e-resp -w '%{http_code}' -X POST -H 'Content-Type: application/json' -d '{"username":"root","password":"nfsense"}' -c /tmp/e2e-cookies https://127.0.0.1:4444/login)
echo "$code"
cat /tmp/e2e-resp"#;

const RPC_SCRIPT: &str = r#"code=$(curl -sk -o /tmp/e2e-resp -w '%{http_code}' -X POST -H 'Content-Type: application/json' -d "$1" -b /tmp/e2e-cookies https://127.0.0.1:4444/api)
echo "$code"
cat /tmp/e2e-resp"#;

pub struct ApiClient;

impl ApiClient {
    /// Split curl's stdout into (http_code, body).
    /// A code of 0 means the connection itself failed (API not up yet).
    fn parse_response(
        what: &str,
        exit: i64,
        out: &str,
        err: &str,
    ) -> Result<(u16, String), String> {
        let (code, body) = out.split_once('\n').unwrap_or((out, ""));
        let code: u16 = code
            .trim()
            .parse()
            .map_err(|_| format!("{what}: unparseable curl output: {out:?}"))?;
        if code == 0 {
            return Err(format!(
                "{what}: connection failed (curl exit {exit}): {err}"
            ));
        }
        Ok((code, body.to_string()))
    }

    /// POST /login with the default admin user; stores the session cookie
    /// in the guest-side cookie jar.
    pub fn login(qga: &mut Qga) -> Result<(), String> {
        let (exit, out, err) =
            qga.exec("/bin/sh", &["-c", LOGIN_SCRIPT], Duration::from_secs(30))?;
        let (code, body) = Self::parse_response("login", exit, &out, &err)?;
        if code == 401 {
            return Err("login rejected (401) — default root/nfsense credentials?".into());
        }
        if !(200..300).contains(&code) {
            return Err(format!("login failed: HTTP {code}: {body}"));
        }
        Ok(())
    }

    /// Wait until the API is up and the session cookie works.
    pub fn wait_ready(
        qga: &mut Qga,
        deadline: Instant,
        mut alive: impl FnMut() -> bool,
    ) -> Result<(), String> {
        let mut last_err = String::from("not attempted");
        while Instant::now() < deadline {
            if !alive() {
                return Err("QEMU exited while waiting for the API".into());
            }
            match Self::login(qga) {
                Ok(()) => return Ok(()),
                Err(e) => last_err = e,
            }
            eprintln!("e2e: waiting for API... ({last_err})");
            std::thread::sleep(Duration::from_secs(2));
        }
        Err(format!("API login never succeeded by deadline: {last_err}"))
    }

    fn call_once(qga: &mut Qga, method: &str, params: &Value) -> Result<Value, String> {
        let body = serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1,
        }))
        .map_err(|e| e.to_string())?;
        let args = vec!["-c", RPC_SCRIPT, "sh", &body];
        let (exit, out, err) = qga.exec("/bin/sh", &args, API_CALL)?;
        let (code, text) = Self::parse_response(method, exit, &out, &err)?;
        if code == 401 {
            return Err("unauthorized (401)".into());
        }
        if !(200..300).contains(&code) {
            return Err(format!("{method}: HTTP {code}: {text}"));
        }
        let v: Value =
            serde_json::from_str(&text).map_err(|e| format!("{method}: bad json: {e}"))?;
        if let Some(err) = v.get("error") {
            return Err(format!("{method}: {err}"));
        }
        Ok(v.get("result").cloned().unwrap_or(Value::Null))
    }

    /// Call a JSON-RPC method; returns `result`, errors on HTTP/RPC
    /// errors. A 401 (stale session, e.g. the server restarted) triggers
    /// one re-login and retry.
    pub fn call(qga: &mut Qga, method: &str, params: Value) -> Result<Value, String> {
        match Self::call_once(qga, method, &params) {
            Err(e) if e.starts_with("unauthorized") => {
                eprintln!("e2e: {method} got 401 — re-login and retry once");
                Self::login(qga)?;
                Self::call_once(qga, method, &params)
            }
            other => other,
        }
    }
}
