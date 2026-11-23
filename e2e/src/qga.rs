//! QGA (QEMU Guest Agent) wire-protocol client over the host side of the
//! virtio-serial chardev unix socket.
//!
//! One JSON object per line each direction, `guest-`-prefixed commands.
//! The chardev socket accepts one host connection at a time; QEMU pairs
//! it with qemu-ga inside the guest as soon as the agent connects (or
//! re-connects after a VM-state restore / reboot).

use std::io::{BufRead, BufReader, BufWriter, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::{Duration, Instant};

use base64::prelude::*;
use serde_json::{json, Value};

pub struct Qga {
    reader: BufReader<UnixStream>,
    writer: BufWriter<UnixStream>,
}

impl Qga {
    /// Open the host side of the chardev socket. The connection may be
    /// accepted by QEMU before qemu-ga inside the guest has attached;
    /// until then reads time out (10 s) — that is the retry signal.
    pub fn connect(sock: &Path) -> Result<Self, String> {
        let stream =
            UnixStream::connect(sock).map_err(|e| format!("connect {}: {e}", sock.display()))?;
        stream.set_read_timeout(Some(Duration::from_secs(10))).ok();
        let reader = BufReader::new(stream.try_clone().map_err(|e| e.to_string())?);
        let writer = BufWriter::new(stream);
        Ok(Self { reader, writer })
    }

    /// Send one command, read one JSON response line, return `return`.
    pub fn cmd(&mut self, name: &str, args: &Value) -> Result<Value, String> {
        let req = json!({ "execute": name, "arguments": args });
        let line = serde_json::to_string(&req).map_err(|e| e.to_string())? + "\n";
        self.writer
            .write_all(line.as_bytes())
            .map_err(|e| e.to_string())?;
        self.writer.flush().map_err(|e| e.to_string())?;

        let mut resp = String::new();
        self.reader
            .read_line(&mut resp)
            .map_err(|e| format!("{name}: {e}"))?;
        let v: Value = serde_json::from_str(resp.trim())
            .map_err(|e| format!("{name}: bad response {resp:?}: {e}"))?;
        if let Some(err) = v.get("error") {
            return Err(format!("{name}: {err}"));
        }
        Ok(v.get("return").cloned().unwrap_or(Value::Null))
    }

    pub fn ping(&mut self) -> Result<(), String> {
        self.cmd("guest-ping", &json!({})).map(|_| ())
    }

    /// Connect + ping until the guest agent is up (or deadline).
    /// Each attempt uses a fresh connection; QEMU re-pairs the channel as
    /// soon as qemu-ga inside the guest connects.
    pub fn wait_ready(
        sock: &Path,
        deadline: Instant,
        mut alive: impl FnMut() -> bool,
    ) -> Result<Self, String> {
        let mut last_err = String::from("not attempted");
        while Instant::now() < deadline {
            if !alive() {
                return Err("QEMU exited while waiting for the guest agent".into());
            }
            match Self::connect(sock).and_then(|mut q| q.ping().map(|_| q)) {
                Ok(q) => return Ok(q),
                Err(e) => last_err = e,
            }
            eprintln!("e2e: waiting for guest agent... ({last_err})");
            std::thread::sleep(Duration::from_secs(2));
        }
        Err(format!("guest agent not ready by deadline: {last_err}"))
    }

    /// Issue `guest-exec` and return the guest pid. Shared by `exec`
    /// (poll to completion) and `exec_fire` (no status collection).
    fn start(&mut self, path: &str, args: &[&str]) -> Result<i64, String> {
        let argv: Vec<Value> = args.iter().map(|a| json!(a)).collect();
        self.cmd(
            "guest-exec",
            &json!({ "path": path, "arg": argv, "capture-output": true }),
        )?
        .get("pid")
        .and_then(Value::as_i64)
        .ok_or_else(|| "guest-exec returned no pid".to_string())
    }

    /// Run a command in the guest and wait for it to finish.
    /// Returns (exitcode, stdout, stderr).
    pub fn exec(
        &mut self,
        path: &str,
        args: &[&str],
        timeout: Duration,
    ) -> Result<(i64, String, String), String> {
        let pid = self.start(path, args)?;

        let deadline = Instant::now() + timeout;
        loop {
            let status = self.cmd("guest-exec-status", &json!({ "pid": pid }))?;
            if status.get("exited").and_then(Value::as_bool) == Some(true) {
                let exit = status.get("exitcode").and_then(Value::as_i64).unwrap_or(-1);
                let out = status
                    .get("out-data")
                    .and_then(Value::as_str)
                    .map(decode_b64)
                    .unwrap_or_default();
                let err = status
                    .get("err-data")
                    .and_then(Value::as_str)
                    .map(decode_b64)
                    .unwrap_or_default();
                return Ok((exit, out, err));
            }
            if Instant::now() > deadline {
                return Err(format!("guest-exec timeout: {path} {args:?}"));
            }
            std::thread::sleep(Duration::from_millis(200));
        }
    }

    /// Start a command in the guest without waiting for it to finish;
    /// returns the guest pid. For commands that take the system down
    /// (e.g. `systemctl reboot`): the QGA channel dies with the
    /// shutdown, so no status can be collected — drop the connection
    /// and wait for the agent to re-pair instead.
    pub fn exec_fire(&mut self, path: &str, args: &[&str]) -> Result<i64, String> {
        self.start(path, args)
    }

    /// Like `exec` but fails when the exit code is not 0.
    pub fn exec_ok(
        &mut self,
        path: &str,
        args: &[&str],
        timeout: Duration,
    ) -> Result<String, String> {
        let (exit, out, err) = self.exec(path, args, timeout)?;
        if exit != 0 {
            return Err(format!("{path} {args:?} exited with {exit}:\n{out}\n{err}"));
        }
        Ok(out)
    }

    /// Power the guest off via its ACPI/shutdown path. The agent socket
    /// closing (an error here) is expected and not a failure.
    pub fn power_off(&mut self) -> Result<(), String> {
        self.cmd("guest-shutdown", &json!({ "mode": "powerdown" }))
            .map(|_| ())
    }
}

fn decode_b64(s: &str) -> String {
    BASE64_STANDARD
        .decode(s)
        .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
        .unwrap_or_default()
}
