//! Minimal QMP (QEMU Machine Protocol) client.
//!
//! Only what the harness needs: capabilities, one request/response
//! exchange that tolerates interleaved async `{"event": ...}` lines, job
//! polling, and the snapshot save/load job pair.
//!
//! QEMU 10 notes (verified against QEMU 10.0.7):
//!  - `snapshot-save` / `snapshot-load` are background *jobs*: they take
//!    `{job-id, tag, vmstate, devices}` and return `{}` immediately.
//!    Completion (and any error) is observed via `query-jobs`; the job's
//!    `error` field is populated on failure.
//!  - `tag` is the snapshot *name*; `vmstate` is the block node that
//!    carries the VM-state (RAM/device) half of the snapshot; `devices`
//!    lists the block nodes whose disk contents are saved/restored.
//!  - `query-snapshots` no longer exists; internal snapshots are not
//!    listed by `query-block` either. Detecting an existing checkpoint
//!    between runs is done host-side with `qemu-img info` (see
//!    `crate::image::overlay_has_snapshot`), before the VM is up.

use std::io::{BufRead, Write};
use std::path::Path;
use std::time::{Duration, Instant};

use serde_json::{json, Value};

use crate::CHECKPOINT;

/// Read deadline for a single QMP response line. Generous on purpose:
/// during a long job, async `JOB_STATUS_CHANGE` events keep the line
/// stream alive, and a dead QEMU is caught separately by the VM liveness
/// check in `crate::ctx`.
const READ_TIMEOUT: Duration = Duration::from_secs(300);

pub struct Qmp {
    reader: std::io::BufReader<std::os::unix::net::UnixStream>,
    writer: std::io::BufWriter<std::os::unix::net::UnixStream>,
    job_seq: u64,
}

impl Qmp {
    /// Connect to the QMP socket, read the greeting, complete the
    /// capability handshake. Retries while QEMU is still coming up.
    pub fn connect(sock: &Path, deadline: Instant) -> Result<Self, String> {
        let mut last_err = String::from("not attempted");
        while Instant::now() < deadline {
            match Self::connect_once(sock) {
                Ok(q) => return Ok(q),
                Err(e) => last_err = e,
            }
            std::thread::sleep(Duration::from_millis(300));
        }
        Err(format!("QMP connect {sock:?} failed: {last_err}"))
    }

    fn connect_once(sock: &Path) -> Result<Self, String> {
        use std::os::unix::net::UnixStream;
        let stream = UnixStream::connect(sock).map_err(|e| e.to_string())?;
        stream.set_read_timeout(Some(READ_TIMEOUT)).ok();
        let mut reader = std::io::BufReader::new(stream.try_clone().map_err(|e| e.to_string())?);
        let writer = std::io::BufWriter::new(stream);

        // Greeting: one line {"QMP": {...}}.
        let mut greeting = String::new();
        reader
            .read_line(&mut greeting)
            .map_err(|e| format!("QMP greeting read: {e}"))?;
        if !greeting.contains("\"QMP\"") {
            return Err(format!("unexpected QMP greeting: {greeting:?}"));
        }

        // Capability handshake.
        let mut self_ = Self {
            reader,
            writer,
            job_seq: 0,
        };
        self_
            .writer
            .write_all(b"{\"execute\": \"qmp_capabilities\"}\n")
            .map_err(|e| e.to_string())?;
        self_.writer.flush().map_err(|e| e.to_string())?;
        let mut line = String::new();
        self_
            .reader
            .read_line(&mut line)
            .map_err(|e| format!("qmp_capabilities read: {e}"))?;
        let v: Value = serde_json::from_str(line.trim()).map_err(|e| e.to_string())?;
        if let Some(err) = v.get("error") {
            return Err(format!("qmp_capabilities failed: {err}"));
        }
        Ok(self_)
    }

    fn next_job_id(&mut self) -> String {
        self.job_seq += 1;
        format!("e2e{}", self.job_seq)
    }

    /// Send one command and read its response, skipping async event lines.
    pub fn cmd(&mut self, name: &str, args: Option<Value>) -> Result<Value, String> {
        let mut req = json!({ "execute": name });
        if let Some(a) = args {
            req.as_object_mut()
                .ok_or("execute object expected")?
                .insert("arguments".into(), a);
        }
        let line = serde_json::to_string(&req).map_err(|e| e.to_string())? + "\n";
        self.writer
            .write_all(line.as_bytes())
            .map_err(|e| e.to_string())?;
        self.writer.flush().map_err(|e| e.to_string())?;

        loop {
            let mut resp = String::new();
            self.reader
                .read_line(&mut resp)
                .map_err(|e| format!("{name}: {e}"))?;
            let v: Value = serde_json::from_str(resp.trim())
                .map_err(|e| format!("{name}: bad response {resp:?}: {e}"))?;
            if v.get("event").is_some() {
                continue; // async event (JOB_STATUS_CHANGE, STOP, RESUME, ...)
            }
            if let Some(err) = v.get("error") {
                return Err(format!("{name}: {err}"));
            }
            return Ok(v.get("return").cloned().unwrap_or(Value::Null));
        }
    }

    /// Poll `query-jobs` until the job with `id` reaches `concluded`;
    /// error if the job reports a failure.
    pub fn wait_job(&mut self, id: &str, deadline: Instant) -> Result<(), String> {
        while Instant::now() < deadline {
            let jobs = self.cmd("query-jobs", None)?;
            if let Some(jobs) = jobs.as_array() {
                for j in jobs {
                    if j.get("id").and_then(Value::as_str) == Some(id) {
                        match j.get("status").and_then(Value::as_str) {
                            Some("concluded") => {
                                if let Some(err) = j.get("error") {
                                    return Err(format!("job {id} failed: {err}"));
                                }
                                return Ok(());
                            }
                            // In-progress QAPI JobStatus values: keep
                            // polling. On a slow disk the first poll can
                            // catch the job mid-work; only terminal or
                            // unknown statuses are fatal.
                            Some("created" | "running" | "waiting" | "paused") => {}
                            Some(other) => {
                                return Err(format!("job {id} in unexpected status {other:?}"));
                            }
                            None => {}
                        }
                    }
                }
            }
            std::thread::sleep(Duration::from_millis(500));
        }
        Err(format!("job {id} did not conclude in time"))
    }

    /// Save the full VM state (RAM + devices + listed disks) as internal
    /// snapshot `tag` on block node `node` (job-based).
    pub fn snapshot_save(&mut self, node: &str, tag: &str) -> Result<(), String> {
        let id = self.next_job_id();
        self.cmd(
            "snapshot-save",
            Some(json!({
                "job-id": id,
                "tag": tag,
                "vmstate": node,
                "devices": [node]
            })),
        )?;
        self.wait_job(&id, Instant::now() + Duration::from_secs(300))
    }

    /// Load internal snapshot `tag` from block node `node` into the
    /// (running) VM (job-based). Pauses the vCPUs for the duration.
    pub fn snapshot_load(&mut self, node: &str, tag: &str) -> Result<(), String> {
        let id = self.next_job_id();
        self.cmd(
            "snapshot-load",
            Some(json!({
                "job-id": id,
                "tag": tag,
                "vmstate": node,
                "devices": [node]
            })),
        )?;
        self.wait_job(&id, Instant::now() + Duration::from_secs(300))
    }

    pub fn query_status(&mut self) -> Result<String, String> {
        Ok(self
            .cmd("query-status", None)?
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string())
    }

    pub fn quit(&mut self) {
        let _ = self.cmd("quit", None);
    }

    /// Convenience: load the harness checkpoint.
    pub fn restore_checkpoint(&mut self) -> Result<(), String> {
        self.snapshot_load(crate::DISK_NODE, CHECKPOINT)
    }
}
