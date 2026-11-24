use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use crate::tpm::Swtpm;

#[derive(Clone)]
pub struct Paths {
    pub qga_sock: PathBuf,
    pub qmp_sock: PathBuf,
    pub serial_log: PathBuf,
    pub serial_sock: PathBuf,
    pub qemu_log: PathBuf,
    pub pidfile: PathBuf,
    pub ovf_vars: PathBuf,
    pub ovf_vars_healthy: PathBuf,
    pub overlay: PathBuf,
    pub smbios_uuid: String,
    pub tpm_state_dir: PathBuf,
    pub tpm_sock: PathBuf,
    pub swtpm_pidfile: PathBuf,
    pub swtpm_log: PathBuf,
}

// Deterministic per image, not random: the kernel command line has
// `systemd.machine_id=firmware` and OVMF's own UUID is all-zero, so without
// a stable UUID the machine-id would change every boot and the `var`
// partition (GPT type derived from the machine-id) would stop matching.
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

// Every VM has all NICs: the device layout is part of the checkpoint, so it
// cannot vary per test.
pub struct Nic {
    pub id: &'static str,
    // The ACPI-index `eno` name wins over the slot name (`ens<slot>`) in
    // systemd's default NamePolicy.
    pub ifname: &'static str,
    pub mac: &'static str,
    pub slot: u8,
}

pub const NICS: &[Nic] = &[
    Nic {
        id: "lan1",
        ifname: "eno16",
        mac: "52:54:00:e2:e0:10",
        slot: 0x10,
    },
    Nic {
        id: "lan2",
        ifname: "eno17",
        mac: "52:54:00:e2:e0:11",
        slot: 0x11,
    },
    Nic {
        id: "wan",
        ifname: "eno18",
        mac: "52:54:00:e2:e0:12",
        slot: 0x12,
    },
];

pub fn nic_ifname(id: &str) -> &'static str {
    NICS.iter()
        .find(|n| n.id == id)
        .map(|n| n.ifname)
        .unwrap_or_else(|| panic!("no NIC {id}"))
}

// Bump whenever the VM device layout changes: an internal snapshot can only
// be loaded into the same devices, so the cache must not be reused.
pub const DEVICE_LAYOUT: &str = "n4";

pub struct Vm {
    child: Child,
    swtpm: Swtpm,
    pub paths: Paths,
}

impl Vm {
    /// Caller must hold the run lock for this cache.
    #[allow(clippy::zombie_processes)]
    pub fn spawn(
        paths: Paths,
        uefi_code: &Path,
        uefi_vars: &Path,
        paused: bool,
    ) -> Result<Self, String> {
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
            // Persistent overlay, not `-snapshot`: the checkpoint lives in it.
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
            // No management NIC: it would interfere with network tests and
            // die with the guest's network when a test breaks it, hiding the
            // failure. Control goes through the guest agent and loopback.
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
            "-chardev".into(),
            format!(
                "socket,id=ser0,path={},server=on,wait=off,logfile={}",
                p.serial_sock.display(),
                p.serial_log.display()
            ),
            "-serial".into(),
            "chardev:ser0".into(),
            "-display".into(),
            "none".into(),
            "-smbios".into(),
            format!("type=1,uuid={}", p.smbios_uuid),
            // Without a TPM the in-guest repart (`Encrypt=tpm2`) fails and
            // `/var` degrades to the root tmpfs, resetting the config every
            // boot. swtpm is the server, so this chardev is a client socket.
            "-chardev".into(),
            format!("socket,id=tpmchar,path={}", p.tpm_sock.display()),
            "-tpmdev".into(),
            "emulator,id=tpm0,chardev=tpmchar".into(),
            "-device".into(),
            "tpm-tis,tpmdev=tpm0".into(),
            "-pidfile".into(),
            p.pidfile.display().to_string(),
        ];
        for nic in NICS {
            args.push("-netdev".into());
            args.push(format!("user,id={},restrict=on", nic.id));
            args.push("-device".into());
            args.push(format!(
                "virtio-net-pci,netdev={id},id=nic-{id},mac={mac},addr={slot:#x},acpi-index={slot}",
                id = nic.id,
                mac = nic.mac,
                slot = nic.slot
            ));
        }
        if paused {
            args.push("-S".into());
        }

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

    pub fn exited(&mut self) -> Option<std::process::ExitStatus> {
        self.child.try_wait().ok().flatten()
    }

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

    pub fn reap(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        self.swtpm.kill();
    }
}

impl Drop for Vm {
    // Artifacts (overlay, logs, pidfile, sockets) are kept for inspection
    // after a failure. A harness killed without unwinding leaves QEMU
    // running with the image locked; the next run kills it by pidfile.
    fn drop(&mut self) {
        self.reap();
    }
}

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
