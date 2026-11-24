use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::time::{Duration, Instant};

pub struct Swtpm {
    child: Child,
    pub socket: PathBuf,
}

impl Swtpm {
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

        // Wait for the socket file, not a connection: a connection would
        // consume the single SWTPM channel that QEMU will take.
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
