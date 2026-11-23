//! Host-side software TPM: the guest gets a real TPM 2.0 from `swtpm`.
//!
//! The image's repart config seals the swap key with `Encrypt=tpm2`, and
//! in-guest `systemd-repart` needs a working TPM 2.0 to finish: without
//! it the `nfsense_var` ext4 partition is never created and `/var` stays
//! on the root tmpfs (config resets at every boot). Most hosts have no
//! hardware TPM, so the harness runs `swtpm` and hands it to QEMU
//! through QEMU's `emulator` TPM backend:
//!
//! ```text
//! -chardev socket,id=tpmchar,path=<cache>/tpm/swtpm-sock
//! -tpmdev emulator,id=tpm0,chardev=tpmchar
//! -device tpm-tis,tpmdev=tpm0
//! ```
//!
//! The `emulator` backend speaks the SWTPM protocol to the external
//! swtpm process: swtpm listens on the unix socket, QEMU connects as the
//! client (so the chardev must NOT be `server=on`). The swtpm state
//! lives under the cache dir, so the TPM's NV memory — sealed keys
//! included — persists across harness invocations, like a real TPM's.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::time::{Duration, Instant};

/// A running `swtpm` process.
pub struct Swtpm {
    child: Child,
    /// The unix socket QEMU connects to (SWTPM protocol).
    pub socket: PathBuf,
}

impl Swtpm {
    /// Start `swtpm socket --tpm2` with state in `state_dir`, listening
    /// on `socket`; wait until the socket file exists.
    pub fn start(
        state_dir: &Path,
        socket: &Path,
        pidfile: &Path,
        log: &Path,
    ) -> Result<Self, String> {
        crate::host::kill_stale_process(pidfile, "swtpm");
        fs::create_dir_all(state_dir).map_err(|e| e.to_string())?;
        fs::remove_file(socket).ok();

        let out = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log)
            .map_err(|e| e.to_string())?;
        let err = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log)
            .map_err(|e| e.to_string())?;
        let mut child = Command::new("swtpm")
            .args([
                "socket",
                "--tpmstate",
                &format!("dir={}", state_dir.display()),
                "--ctrl",
                &format!("type=unixio,path={}", socket.display()),
                "--tpm2",
            ])
            .stdout(std::process::Stdio::from(out))
            .stderr(std::process::Stdio::from(err))
            .spawn()
            .map_err(|e| format!("failed to spawn swtpm: {e}"))?;
        let _ = fs::write(pidfile, child.id().to_string());

        // swtpm binds the socket a few milliseconds after start; waiting
        // for the *file* (not a connection) avoids consuming the single
        // SWTPM channel that QEMU will take.
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            if socket.exists() {
                break;
            }
            if Instant::now() > deadline {
                let _ = child.kill();
                return Err("swtpm socket did not appear within 10s".into());
            }
            std::thread::sleep(Duration::from_millis(50));
        }
        eprintln!("e2e: swtpm started (pid {})", child.id());
        Ok(Self {
            child,
            socket: socket.to_path_buf(),
        })
    }

    /// Kill swtpm and reap it (state stays in `state_dir`).
    pub fn kill(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

impl Drop for Swtpm {
    fn drop(&mut self) {
        self.kill();
    }
}
