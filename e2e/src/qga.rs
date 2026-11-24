use std::io::{BufRead, BufReader, BufWriter, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use base64::prelude::*;
use serde_json::{json, Value};

pub struct Qga {
    reader: BufReader<UnixStream>,
    writer: BufWriter<UnixStream>,
    // Set after any I/O or protocol error: a reply arriving after a read
    // timeout would otherwise be taken as the answer to the next command,
    // shifting every later reply by one. Reconnecting re-syncs instead.
    broken: bool,
}

impl Qga {
    // QEMU may accept the connection before qemu-ga in the guest has
    // attached; until then reads time out, which is the retry signal.
    pub fn connect(sock: &Path) -> Result<Self, String> {
        let stream =
            UnixStream::connect(sock).map_err(|e| format!("connect {}: {e}", sock.display()))?;
        stream.set_read_timeout(Some(Duration::from_secs(10))).ok();
        let reader = BufReader::new(stream.try_clone().map_err(|e| e.to_string())?);
        let writer = BufWriter::new(stream);
        let mut q = Self {
            reader,
            writer,
            broken: false,
        };
        q.sync()?;
        Ok(q)
    }

    // `guest-sync-delimited`: qemu-ga answers with a 0xFF byte before the
    // reply, so everything before it (stale replies, a half-read line from a
    // previous host connection) can be discarded to realign the stream.
    fn sync(&mut self) -> Result<(), String> {
        static SEQ: AtomicU64 = AtomicU64::new(1);
        let id = (std::process::id() as u64) << 32 | SEQ.fetch_add(1, Ordering::Relaxed);
        // A leading 0xFF resets qemu-ga's JSON parser on the guest side.
        let mut w = vec![0xffu8];
        w.extend_from_slice(
            json!({ "execute": "guest-sync-delimited", "arguments": { "id": id } })
                .to_string()
                .as_bytes(),
        );
        w.push(b'\n');
        self.writer
            .write_all(&w)
            .map_err(|e| self.fail(format!("sync: {e}")))?;
        self.writer
            .flush()
            .map_err(|e| self.fail(format!("sync: {e}")))?;
        loop {
            let mut junk = Vec::new();
            self.reader
                .read_until(0xff, &mut junk)
                .map_err(|e| self.fail(format!("sync: {e}")))?;
            if junk.last() != Some(&0xff) {
                return Err(self.fail("sync: connection closed".into()));
            }
            let mut line = String::new();
            self.reader
                .read_line(&mut line)
                .map_err(|e| self.fail(format!("sync: {e}")))?;
            let v: Value = match serde_json::from_str(line.trim()) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if v.get("return").and_then(Value::as_u64) == Some(id) {
                return Ok(());
            }
        }
    }

    pub fn is_broken(&self) -> bool {
        self.broken
    }

    fn fail(&mut self, msg: String) -> String {
        self.broken = true;
        msg
    }

    pub fn cmd(&mut self, name: &str, args: &Value) -> Result<Value, String> {
        if self.broken {
            return Err(format!(
                "{name}: guest agent connection is broken, reconnect"
            ));
        }
        let req = json!({ "execute": name, "arguments": args });
        let line = serde_json::to_string(&req).map_err(|e| e.to_string())? + "\n";
        self.writer
            .write_all(line.as_bytes())
            .map_err(|e| self.fail(format!("{name}: {e}")))?;
        self.writer
            .flush()
            .map_err(|e| self.fail(format!("{name}: {e}")))?;

        let mut resp = String::new();
        match self.reader.read_line(&mut resp) {
            Ok(0) => return Err(self.fail(format!("{name}: connection closed"))),
            Ok(_) => {}
            Err(e) => return Err(self.fail(format!("{name}: {e}"))),
        }
        let v: Value = match serde_json::from_str(resp.trim()) {
            Ok(v) => v,
            Err(e) => return Err(self.fail(format!("{name}: bad response {resp:?}: {e}"))),
        };
        if let Some(err) = v.get("error") {
            return Err(format!("{name}: {err}"));
        }
        Ok(v.get("return").cloned().unwrap_or(Value::Null))
    }

    pub fn ping(&mut self) -> Result<(), String> {
        self.cmd("guest-ping", &json!({})).map(|_| ())
    }

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

    // qemu-ga takes the argv under `arg`, not `arguments`.
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

    // No status collection: for commands that take the system down (e.g.
    // `systemctl reboot`) the QGA channel dies with the shutdown.
    pub fn exec_fire(&mut self, path: &str, args: &[&str]) -> Result<i64, String> {
        self.start(path, args)
    }

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

    /// The agent socket closing (an error here) is expected, not a failure.
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
