use std::io::{BufRead, Write};
use std::path::Path;
use std::time::{Duration, Instant};

use serde_json::{json, Value};

use crate::CHECKPOINT;

// Generous on purpose: async `JOB_STATUS_CHANGE` events keep the stream alive
// during a long job, and a dead QEMU is caught by the VM liveness check.
const READ_TIMEOUT: Duration = Duration::from_secs(300);

pub struct Qmp {
    reader: std::io::BufReader<std::os::unix::net::UnixStream>,
    writer: std::io::BufWriter<std::os::unix::net::UnixStream>,
    job_seq: u64,
}

impl Qmp {
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

        let mut greeting = String::new();
        reader
            .read_line(&mut greeting)
            .map_err(|e| format!("QMP greeting read: {e}"))?;
        if !greeting.contains("\"QMP\"") {
            return Err(format!("unexpected QMP greeting: {greeting:?}"));
        }

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
                continue;
            }
            if let Some(err) = v.get("error") {
                return Err(format!("{name}: {err}"));
            }
            return Ok(v.get("return").cloned().unwrap_or(Value::Null));
        }
    }

    // snapshot-save/-load are jobs in QEMU 10: the command returns
    // immediately, and completion and errors only show up in `query-jobs`.
    pub fn wait_job(&mut self, id: &str, deadline: Instant) -> Result<(), String> {
        while Instant::now() < deadline {
            let jobs = self.cmd("query-jobs", None)?;
            if let Some(jobs) = jobs.as_array() {
                for j in jobs {
                    if j.get("id").and_then(Value::as_str) == Some(id) {
                        match j.get("status").and_then(Value::as_str) {
                            Some("concluded") => {
                                let err = j.get("error").cloned();
                                // Snapshot jobs are manual-dismiss: without
                                // this they pile up in `query-jobs`.
                                let _ = self.cmd("job-dismiss", Some(json!({ "id": id })));
                                if let Some(err) = err {
                                    return Err(format!("job {id} failed: {err}"));
                                }
                                return Ok(());
                            }
                            // On a slow disk the first poll can catch the
                            // job mid-work; only terminal or unknown
                            // statuses are fatal.
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

    pub fn set_link(&mut self, id: &str, up: bool) -> Result<(), String> {
        self.cmd(
            "set_link",
            Some(serde_json::json!({"name": format!("nic-{id}"), "up": up})),
        )
        .map(|_| ())
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

    pub fn restore_checkpoint(&mut self) -> Result<(), String> {
        self.snapshot_load(crate::DISK_NODE, CHECKPOINT)
    }
}
