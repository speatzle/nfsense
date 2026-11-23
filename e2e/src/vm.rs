//! QEMU process management.
//!
//! The VM is spawned against a *persistent* qcow2 overlay (owned by the
//! cache, see `crate::image`), so disk state survives across runs. A
//! full internal VM snapshot (`crate::CHECKPOINT`) inside that overlay is
//! the checkpoint used for fast test-to-test restore (see
//! `crate::qmp::Qmp::snapshot_load`).
//!
//! Debug artifacts (serial log, QEMU stderr, pidfile, overlay) are kept
//! on failure so a crashed run can be inspected — the previous harness
//! deleted them in `Drop`, destroying the evidence.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use crate::tpm::Swtpm;

/// All paths a spawn touches, kept together so spawn/cleanup/inspection
/// agree.
#[derive(Clone)]
pub struct Paths {
    pub qga_sock: PathBuf,
    pub qmp_sock: PathBuf,
    pub serial_log: PathBuf,
    pub qemu_log: PathBuf,
    pub pidfile: PathBuf,
    /// Fresh copy of the OVMF_VARS template for this spawn (the vars are
    /// scratch: boot firmware writes to them every run).
    pub ovf_vars: PathBuf,
    /// Read-only OVMF code firmware (system path, copied nowhere).
    pub ovf_code: PathBuf,
    /// The persistent overlay image (backed by the pristine base image).
    pub overlay: PathBuf,
    /// Stable SMBIOS system UUID for this image cache. The kernel
    /// command line carries `systemd.machine_id=firmware`, so PID 1
    /// derives the machine-id from the firmware SMBIOS UUID. OVMF's
    /// own UUID is all-zero (invalid), which would make the machine-id
    /// random on every boot — fatal for the `Type=var` repart
    /// partition (its GPT type GUID is derived from the machine-id,
    /// and the automount only matches the *current* machine-id).
    pub smbios_uuid: String,
    /// swtpm state dir — the guest's TPM 2.0, persisted across runs
    /// (see `crate::tpm`).
    pub tpm_state_dir: PathBuf,
    /// swtpm's SWTPM-protocol unix socket; QEMU connects to it as the
    /// client (`-tpmdev emulator` + `tpm-tis`).
    pub tpm_sock: PathBuf,
    pub swtpm_pidfile: PathBuf,
    pub swtpm_log: PathBuf,
}

/// Deterministic (per image id) SMBIOS UUID, v4-shaped. Not random on
/// purpose: the machine-id must be stable across boots of the same
/// image, so the machine-id-derived GPT partition types (e.g. `var`)
/// keep matching.
pub fn smbios_uuid_from(image_id: &str) -> String {
    fn fnv1a64(mut h: u64, data: &[u8]) -> u64 {
        for &b in data {
            h ^= b as u64;
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
        h
    }
    let id = image_id.as_bytes();
    let lo = fnv1a64(0xcbf2_9ce4_8422_2325, id);
    let mut salted = id.to_vec();
    salted.push(b'!');
    let hi = fnv1a64(0xcbf2_9ce4_8422_2325, &salted);
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&lo.to_le_bytes());
    bytes[8..].copy_from_slice(&hi.to_le_bytes());
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    let mut s = String::new();
    for (i, b) in bytes.iter().enumerate() {
        if matches!(i, 4 | 6 | 8 | 10) {
            s.push('-');
        }
        let _ = std::write!(&mut s, "{b:02x}");
    }
    s
}

pub struct Vm {
    child: Child,
    swtpm: Swtpm,
    pub paths: Paths,
}

impl Vm {
    /// Spawn QEMU. Caller must hold the run lock for this cache.
    #[allow(clippy::zombie_processes)]
    pub fn spawn(paths: Paths, uefi_code: &Path, uefi_vars: &Path) -> Result<Self, String> {
        fs::remove_file(&paths.pidfile).ok();
        let qemu_log = fs::File::create(&paths.qemu_log).map_err(|e| e.to_string())?;

        // The TPM must be up before QEMU starts: QEMU's `emulator` TPM
        // backend connects to the swtpm socket at spawn time.
        let swtpm = Swtpm::start(
            &paths.tpm_state_dir,
            &paths.tpm_sock,
            &paths.swtpm_pidfile,
            &paths.swtpm_log,
        )?;

        let p = &paths;
        const DISK0_NODE: &str = "driver=qcow2,node-name=disk0,file=d0f";
        let mut args: Vec<String> = vec![
            "-enable-kvm".into(),
            "-m".into(),
            "2048".into(),
            "-smp".into(),
            "2".into(),
            "-drive".into(),
            format!(
                "if=pflash,format=raw,readonly=on,file={}",
                uefi_code.display()
            ),
            "-drive".into(),
            format!("if=pflash,format=raw,file={}", uefi_vars.display()),
            // Persistent overlay (NOT `-snapshot`): writes land in the
            // cache overlay; the internal `healthy` snapshot inside it is
            // the checkpoint.
            // Explicit block node names: the `vmstate`/`devices`
            // arguments of QMP snapshot-save/snapshot-load address node
            // names, and a plain `-drive` leaves the auto-generated node
            // name opaque.
            "-blockdev".into(),
            format!("driver=file,node-name=d0f,filename={}", p.overlay.display()),
            "-blockdev".into(),
            DISK0_NODE.to_string(),
            "-device".into(),
            "ide-hd,drive=disk0".into(),
            // No network devices: the guest is reached through the
            // guest-agent socket (control) and loopback (API). Test
            // suites that need data-plane NICs attach them as fixtures.
            "-chardev".into(),
            format!(
                "socket,id=char0,path={},server=on,wait=off",
                p.qga_sock.display()
            ),
            "-device".into(),
            "virtio-serial-pci".into(),
            "-device".into(),
            "virtserialport,name=org.qemu.guest_agent.0,chardev=char0".into(),
            "-qmp".into(),
            format!("unix:{},server=on,wait=off", p.qmp_sock.display()),
            "-serial".into(),
            format!("file:{}", p.serial_log.display()),
            "-display".into(),
            "none".into(),
            // Stable SMBIOS system UUID → stable machine-id
            // (`systemd.machine_id=firmware`); see `Paths::smbios_uuid`.
            "-smbios".into(),
            format!("type=1,uuid={}", p.smbios_uuid),
            // TPM 2.0 for in-guest repart (`Encrypt=tpm2` → persistent
            // `nfsense_var` ext4 partition → `/var`): swtpm behind
            // QEMU's `emulator` backend (SWTPM protocol). The chardev
            // is a *client* socket — swtpm is the server.
            "-chardev".into(),
            format!("socket,id=tpmchar,path={}", p.tpm_sock.display()),
            "-tpmdev".into(),
            "emulator,id=tpm0,chardev=tpmchar".into(),
            "-device".into(),
            "tpm-tis,tpmdev=tpm0".into(),
            "-pidfile".into(),
            p.pidfile.display().to_string(),
        ];
        args.shrink_to_fit();

        let mut cmd = Command::new("qemu-system-x86_64");
        cmd.args(&args);
        let mut child = cmd
            .stdout(Stdio::null())
            .stderr(Stdio::from(qemu_log))
            .spawn()
            .map_err(|e| format!("failed to spawn QEMU: {e}"))?;

        // QEMU dies instantly on bad args; surface that now instead of
        // letting the QMP connect time out minutes later.
        std::thread::sleep(Duration::from_millis(400));
        if let Some(status) = child.try_wait().ok().flatten() {
            let log = tail(&paths.qemu_log, 1200);
            return Err(format!("QEMU exited early ({status}):\n{log}"));
        }

        eprintln!("e2e: QEMU started (pid {})", child.id());
        Ok(Self {
            child,
            swtpm,
            paths,
        })
    }

    /// Poll without blocking; `Some(status)` means QEMU has exited.
    pub fn exited(&mut self) -> Option<std::process::ExitStatus> {
        self.child.try_wait().ok().flatten()
    }

    /// Is QEMU still running?
    pub fn alive(&mut self) -> bool {
        self.exited().is_none()
    }

    pub fn wait_exited(&mut self, deadline: Instant) -> Result<bool, String> {
        loop {
            if let Some(s) = self.exited() {
                return Ok(s.success());
            }
            if Instant::now() > deadline {
                return Ok(false);
            }
            std::thread::sleep(Duration::from_millis(500));
        }
    }

    /// Kill QEMU (and swtpm) and reap them.
    pub fn reap(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        self.swtpm.kill();
    }
}

impl Drop for Vm {
    /// Kill QEMU but keep every artifact (overlay, logs, pidfile, socket
    /// files) in the cache dir for inspection. The kernel frees the
    /// image lock and the run-lock flock when the process dies, so a
    /// killed run can never wedge the next one.
    fn drop(&mut self) {
        self.reap();
    }
}

/// Last `n` chars of a log file (diagnostics on failure).
pub fn tail(path: &Path, n: usize) -> String {
    fs::read_to_string(path)
        .unwrap_or_default()
        .chars()
        .rev()
        .take(n)
        .collect::<String>()
        .chars()
        .rev()
        .collect()
}
