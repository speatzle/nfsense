use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use fs2::FileExt;

use crate::checks;
use crate::host;
use crate::image::{self, BaseImage};
use crate::qga::Qga;
use crate::qmp::Qmp;
use crate::timeouts::AGENT_RECONNECT;
use crate::vm::{self, Paths, Vm};
use crate::{CHECKPOINT, DISK_NODE};

// Skip: the host cannot run e2e at all; tests pass with a note so KVM-less
// CI stays green. Fail: the prerequisites were present, so a broken image
// must not pass as "skipped".
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitError {
    Skip(String),
    Fail(String),
}

impl std::fmt::Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Skip(r) => write!(f, "e2e SKIP: {r}"),
            Self::Fail(r) => write!(f, "e2e FAIL: {r}"),
        }
    }
}

pub struct Ctx {
    pub vm: Vm,
    pub qmp: Qmp,
    pub qga: Option<Qga>,
    pub base: BaseImage,
    pub cache: PathBuf,
    uefi_code: PathBuf,
    uefi_vars: PathBuf,
    _lock: fs::File,
}

impl Ctx {
    pub fn try_new(dir: Option<&Path>) -> Result<Ctx, InitError> {
        let prereqs = host::check_prereqs().map_err(InitError::Skip)?;
        let base = image::find_base_image(dir).map_err(InitError::Skip)?;
        let cache = image::cache_dir(&base);
        image::check_socket_path_len(&cache).map_err(InitError::Fail)?;
        fs::create_dir_all(&cache)
            .map_err(|e| InitError::Fail(format!("create {}: {e}", cache.display())))?;

        let lock = acquire_lock(&cache).map_err(InitError::Fail)?;
        host::kill_stale_qemu(&cache.join("qemu.pid"));

        let paths = Paths::for_cache(&cache, &base.id);

        let overlay = image::ensure_overlay(&base, &cache).map_err(InitError::Fail)?;
        // The OVMF vars are copied after the snapshot is saved, so a run
        // interrupted in between leaves no usable checkpoint.
        let has_cp = image::overlay_has_snapshot(&overlay, CHECKPOINT)
            .map_err(InitError::Fail)?
            && paths.ovf_vars_healthy.exists();

        if has_cp {
            eprintln!(
                "e2e: checkpoint '{}' found in {} — fast restore path",
                CHECKPOINT,
                overlay.display()
            );
            match Self::spawn_and_load(
                &prereqs,
                &base,
                &cache,
                &paths,
                lock.try_clone()
                    .map_err(|e| InitError::Fail(e.to_string()))?,
            ) {
                Ok(ctx) => return Ok(ctx),
                // Only a checkpoint that QEMU could not load or that did not
                // come back up is thrown away; host and spawn errors would
                // hit the cold boot as well.
                Err(e) if e.starts_with(CHECKPOINT_BROKEN) => {
                    eprintln!("e2e: {e}\ne2e: falling back to a cold boot");
                }
                Err(e) => return Err(InitError::Fail(e)),
            }
        }

        // An overlay without a complete checkpoint may be half booted by an
        // interrupted run; the checkpoint must come from a first boot.
        fs::remove_file(&overlay).ok();
        fs::remove_file(&paths.ovf_vars_healthy).ok();
        let _ = fs::remove_dir_all(&paths.tpm_state_dir);
        let overlay = image::ensure_overlay(&base, &cache).map_err(InitError::Fail)?;
        Self::cold_boot(&prereqs, &base, &cache, &paths, &overlay, lock).map_err(InitError::Fail)
    }

    // Respawn instead of loading in place, so whatever the previous test did
    // to QEMU (crash, power-off, a hung QGA channel) cannot leak into the next.
    pub fn restore(&mut self) -> Result<(), String> {
        self.qga = None;
        self.vm.reap();
        let (vm, qmp, qga) =
            spawn_from_checkpoint(&self.vm.paths, &self.uefi_code, &self.uefi_vars)?;
        self.vm = vm;
        self.qmp = qmp;
        self.qga = Some(qga);
        Ok(())
    }

    // In-guest `systemctl reboot`, not a QMP reset: failover and sysupdate
    // reboots are guest-initiated with a clean shutdown.
    pub fn reboot(&mut self) -> Result<(), String> {
        self.reboot_unsettled()?;
        self.wait_settled()
    }

    // Completion is detected by a changed boot id: qemu-ga can still answer
    // during the shutdown window, so a ping would mistake a guest about to go
    // down for a rebooted one.
    pub fn reboot_unsettled(&mut self) -> Result<(), String> {
        eprintln!("e2e: guest rebooting (graceful: systemctl reboot)...");
        let qga = self.agent()?;
        let old_boot_id = qga
            .exec_ok(
                "cat",
                &["/proc/sys/kernel/random/boot_id"],
                Duration::from_secs(15),
            )?
            .trim()
            .to_string();
        // Fire-and-forget: the system goes down before a status could be
        // collected, and the channel dies with it.
        qga.exec_fire("/bin/systemctl", &["reboot"])?;
        self.qga = None;

        let deadline = Instant::now() + AGENT_RECONNECT;
        loop {
            if !self.vm.alive() {
                return Err("QEMU exited during reboot (expected an in-place reset)".into());
            }
            let sock = self.vm.paths.qga_sock.clone();
            if let Ok(mut q) = Qga::connect(&sock) {
                match q.exec_ok(
                    "cat",
                    &["/proc/sys/kernel/random/boot_id"],
                    Duration::from_secs(10),
                ) {
                    Ok(id) if id.trim() != old_boot_id => {
                        eprintln!("e2e: guest rebooted (new boot id {})", id.trim());
                        self.qga = Some(q);
                        return Ok(());
                    }
                    // guest-exec worked in the old boot (we just used it), so
                    // a refusal means the new boot's config did not enable it
                    // (e.g. a broken config). See `restore_guest_exec`.
                    Err(e) if exec_refused(&e) => {
                        eprintln!("e2e: guest rebooted (guest-exec is disabled in the new boot)");
                        self.qga = Some(q);
                        return Ok(());
                    }
                    _ => {}
                }
            }
            if Instant::now() >= deadline {
                return Err(format!(
                    "guest did not reboot (boot id unchanged) by deadline\n--- serial tail ---\n{}",
                    vm::tail(&self.vm.paths.serial_log, 2000)
                ));
            }
            std::thread::sleep(Duration::from_secs(2));
        }
    }

    // Over the serial console because qemu-ga refuses exec in this boot. Only
    // qemu-ga's environment file is touched (not the firewall), so the boot
    // under test stays as it was.
    pub fn restore_guest_exec(&mut self) -> Result<(), String> {
        let mut ser = crate::serial::Serial::connect(&self.vm.paths.serial_sock)?;
        ser.login("root", "nfsense", Duration::from_secs(300))?;
        let (code, out) = ser.run(
            "mkdir -p /run/nfsense && \
             echo 'NFSENSE_QGA_EXTRA_RPCS=,guest-exec,guest-exec-status' > /run/nfsense/qemu-ga.env && \
             systemctl restart qemu-guest-agent.service",
            Duration::from_secs(60),
        )?;
        ser.run("exit", Duration::from_secs(5)).ok();
        if code != 0 {
            return Err(format!(
                "restoring guest-exec over serial failed ({code}):\n{out}"
            ));
        }
        self.qga = None;
        let deadline = Instant::now() + AGENT_RECONNECT;
        loop {
            if let Ok(mut q) = Qga::connect(&self.vm.paths.qga_sock) {
                if q.exec_ok("/bin/true", &[], Duration::from_secs(10)).is_ok() {
                    self.qga = Some(q);
                    return Ok(());
                }
            }
            if Instant::now() >= deadline {
                return Err("guest-exec did not come back after restarting qemu-ga".into());
            }
            std::thread::sleep(Duration::from_secs(2));
        }
    }

    // Power loss: stronger than `reboot()` for persistence checks. The `sync`
    // first makes deliberate writes (e.g. a just-applied config) reach disk;
    // otherwise the reset would drop them from the guest page cache.
    pub fn hard_reset(&mut self) -> Result<(), String> {
        eprintln!("e2e: simulating power loss (system_reset)...");
        let qga = self.agent()?;
        qga.exec_ok("sync", &[], Duration::from_secs(30))?;
        self.qmp.cmd("system_reset", None)?;
        self.qga = None;
        self.agent()?;
        self.wait_settled()
    }

    pub fn set_link(&mut self, id: &str, up: bool) -> Result<(), String> {
        self.qmp.set_link(id, up)
    }

    // The guest agent answers long before nfsense is up; wait for the API so
    // the caller's first call does not race the boot.
    pub fn wait_settled(&mut self) -> Result<(), String> {
        let qga = self.agent()?;
        checks::wait_unit_active(qga, "nfsense")?;
        let qga = self.qga.as_mut().ok_or("guest agent lost")?;
        checks::api_ready(qga, || true)
    }

    pub fn agent(&mut self) -> Result<&mut Qga, String> {
        let pinged = self.qga.as_mut().map(|q| q.ping());
        match pinged {
            Some(Ok(())) => Ok(self.qga.as_mut().unwrap()),
            _ => {
                self.qga = None;
                let sock = self.vm.paths.qga_sock.clone();
                let q =
                    Qga::wait_ready(&sock, Instant::now() + AGENT_RECONNECT, || self.vm.alive())
                        .map_err(|e| {
                            format!(
                                "agent reconnect failed: {e}\n--- serial tail ---\n{}",
                                vm::tail(&self.vm.paths.serial_log, 2000)
                            )
                        })?;
                self.qga = Some(q);
                Ok(self.qga.as_mut().unwrap())
            }
        }
    }

    fn spawn_and_load(
        prereqs: &host::Prereqs,
        base: &BaseImage,
        cache: &Path,
        paths: &Paths,
        lock: fs::File,
    ) -> Result<Ctx, String> {
        let (vm, qmp, qga) = spawn_from_checkpoint(paths, &prereqs.uefi_code, &prereqs.uefi_vars)?;
        Ok(Ctx {
            vm,
            qmp,
            qga: Some(qga),
            base: base.clone(),
            cache: cache.to_path_buf(),
            uefi_code: prereqs.uefi_code.clone(),
            uefi_vars: prereqs.uefi_vars.clone(),
            _lock: lock,
        })
    }

    fn cold_boot(
        prereqs: &host::Prereqs,
        base: &BaseImage,
        cache: &Path,
        paths: &Paths,
        overlay: &Path,
        lock: fs::File,
    ) -> Result<Ctx, String> {
        prepare_spawn(paths, &prereqs.uefi_vars)?;
        eprintln!(
            "e2e: cold booting {} (first run for this image — takes a few minutes)",
            overlay.display()
        );
        let mut vm = Vm::spawn(paths.clone(), &prereqs.uefi_code, &paths.ovf_vars, false)?;
        let mut qmp = Qmp::connect(&paths.qmp_sock, Instant::now() + Duration::from_secs(30))?;

        let qga = Qga::wait_ready(
            &paths.qga_sock,
            Instant::now() + crate::timeouts::BOOT,
            || vm.alive(),
        )
        .map_err(|e| {
            format!(
                "{e}\n\n--- serial tail ---\n{}\n--- qemu log tail ---\n{}",
                vm::tail(&paths.serial_log, 2000),
                vm::tail(&paths.qemu_log, 1500)
            )
        })?;
        let mut qga = bootstrap_exec(qga, paths, || vm.alive())?;
        checks::wait_unit_active(&mut qga, "nfsense")?;
        checks::wait_boot_jobs_drained(&mut qga)?;
        checks::api_ready(&mut qga, || vm.alive())?;

        // Flush the guest's page cache so the checkpoint captures a
        // consistent on-disk state (first-boot config writes included).
        qga.exec_ok("sync", &[], Duration::from_secs(30))?;

        eprintln!("e2e: saving checkpoint '{CHECKPOINT}'...");
        qmp.snapshot_save(DISK_NODE, CHECKPOINT)?;
        // The OVMF vars are not part of the snapshot (raw pflash, not a
        // snapshot-capable node). Keep the ones this boot used, so a
        // respawn pairs the checkpoint with matching firmware variables
        // instead of a pristine template.
        fs::copy(&paths.ovf_vars, &paths.ovf_vars_healthy)
            .map_err(|e| format!("save OVMF vars: {e}"))?;
        eprintln!("e2e: checkpoint saved — subsequent runs restore from it");

        Ok(Ctx {
            vm,
            qmp,
            qga: Some(qga),
            base: base.clone(),
            cache: cache.to_path_buf(),
            uefi_code: prereqs.uefi_code.clone(),
            uefi_vars: prereqs.uefi_vars.clone(),
            _lock: lock,
        })
    }

    pub fn shutdown(&mut self) {
        self.qmp.quit();
        if !self
            .vm
            .wait_exited(Instant::now() + Duration::from_secs(30))
            .unwrap_or(false)
        {
            eprintln!("e2e: QMP quit did not stop QEMU in 30s — killing");
            self.vm.reap();
        }
        self.qga = None;
    }
}

// Images with the guest-agent policy refuse guest-exec until an admin opts
// in. Opt in the way an admin would (serial login with the default
// credentials, API on loopback, apply) once before the checkpoint is saved,
// so every test starts opted in.
fn exec_refused(e: &str) -> bool {
    e.contains("not allowed") || e.contains("has been disabled")
}

fn bootstrap_exec(
    mut qga: Qga,
    paths: &Paths,
    mut alive: impl FnMut() -> bool,
) -> Result<Qga, String> {
    match qga.exec("/bin/true", &[], Duration::from_secs(10)) {
        Ok(_) => return Ok(qga),
        Err(e) if exec_refused(&e) => {
            eprintln!(
                "e2e: guest-exec is refused (production policy) — opting in via the serial console"
            );
        }
        Err(e) => return Err(format!("probing guest-exec: {e}")),
    }
    drop(qga);

    let mut ser = crate::serial::Serial::connect(&paths.serial_sock)?;
    ser.login("root", "nfsense", Duration::from_secs(300))?;
    // The API may still be starting; retry the login until it answers.
    // A subshell, so `exit` does not log out the serial login shell.
    let script = r#"(j='-sk -H Content-Type:application/json -b /tmp/e2e-boot-cookies -c /tmp/e2e-boot-cookies'
i=0; until curl $j -o /dev/null -f -d '{"username":"root","password":"nfsense"}' https://127.0.0.1:4444/login; do
  i=$((i+1)); [ $i -gt 90 ] && exit 3; sleep 2; done
rpc() { curl $j -sS -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"$1\",\"params\":$2}" https://127.0.0.1:4444/api; echo; }
rpc system.settings.set '{"guest_agent":{"allow_exec":true}}'
rpc config.pending.apply '{}'
rm -f /tmp/e2e-boot-cookies)"#;
    let (code, out) = ser.run(script, Duration::from_secs(400))?;
    if code != 0 || out.contains("\"error\"") {
        return Err(format!(
            "opting in to guest-exec over the serial console failed (exit {code}):\n{out}"
        ));
    }
    ser.run("exit", Duration::from_secs(5)).ok();

    // Apply restarted qemu-ga with the new policy, dropping the channel.
    let deadline = Instant::now() + AGENT_RECONNECT;
    loop {
        let mut q = Qga::wait_ready(&paths.qga_sock, deadline, &mut alive)?;
        match q.exec("/bin/true", &[], Duration::from_secs(10)) {
            Ok(_) => {
                eprintln!("e2e: guest-exec enabled");
                return Ok(q);
            }
            Err(e) if Instant::now() > deadline => {
                return Err(format!("guest-exec still refused after opting in: {e}"))
            }
            Err(_) => std::thread::sleep(Duration::from_secs(2)),
        }
    }
}

const CHECKPOINT_BROKEN: &str = "checkpoint restore failed";

fn spawn_from_checkpoint(
    paths: &Paths,
    uefi_code: &Path,
    uefi_vars_template: &Path,
) -> Result<(Vm, Qmp, Qga), String> {
    let vars = if paths.ovf_vars_healthy.exists() {
        paths.ovf_vars_healthy.as_path()
    } else {
        uefi_vars_template
    };
    prepare_spawn(paths, vars)?;
    let mut vm = Vm::spawn(paths.clone(), uefi_code, &paths.ovf_vars, true)?;
    let mut qmp = Qmp::connect(&paths.qmp_sock, Instant::now() + Duration::from_secs(30))?;
    qmp.restore_checkpoint().map_err(|e| {
        format!(
            "{CHECKPOINT_BROKEN}: snapshot-load failed: {e}\n--- qemu log tail ---\n{}",
            vm::tail(&paths.qemu_log, 1500)
        )
    })?;
    qmp.cmd("cont", None)?;
    let qga = Qga::wait_ready(&paths.qga_sock, Instant::now() + AGENT_RECONNECT, || {
        vm.alive()
    })
    .map_err(|e| {
        format!(
            "{CHECKPOINT_BROKEN}: agent not reachable after snapshot-load: {e}\n--- serial tail ---\n{}",
            vm::tail(&paths.serial_log, 2000)
        )
    })?;
    Ok((vm, qmp, qga))
}

fn prepare_spawn(paths: &Paths, uefi_vars: &Path) -> Result<(), String> {
    fs::copy(uefi_vars, &paths.ovf_vars).map_err(|e| {
        format!(
            "copy OVMF_VARS template to {}: {e}",
            paths.ovf_vars.display()
        )
    })?;
    fs::remove_file(&paths.qga_sock).ok();
    fs::remove_file(&paths.qmp_sock).ok();
    fs::remove_file(&paths.serial_sock).ok();
    fs::File::create(&paths.serial_log).map_err(|e| e.to_string())?;
    Ok(())
}

// Retried because another harness may legitimately hold the lock for minutes
// (cold boot). The kernel releases it when the holder dies, so this only
// blocks on a live harness.
fn acquire_lock(cache: &Path) -> Result<fs::File, String> {
    let path = cache.join("vm.lock");
    let file = fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(&path)
        .map_err(|e| format!("open {}: {e}", path.display()))?;
    let deadline = Instant::now() + Duration::from_secs(600);
    loop {
        match file.try_lock_exclusive() {
            Ok(()) => return Ok(file),
            Err(_) if Instant::now() < deadline => {
                eprintln!(
                    "e2e: another e2e run holds the lock on {} — waiting...",
                    path.display()
                );
                std::thread::sleep(Duration::from_secs(5));
            }
            Err(e) => {
                return Err(format!(
                    "could not lock {} (another e2e harness is running?): {e}",
                    path.display()
                ))
            }
        }
    }
}

impl Paths {
    pub fn for_cache(cache: &Path, image_id: &str) -> Self {
        Self {
            qga_sock: cache.join("ga.sock"),
            qmp_sock: cache.join("mp.sock"),
            serial_log: cache.join("serial.log"),
            serial_sock: cache.join("ser.sock"),
            qemu_log: cache.join("qemu.log"),
            pidfile: cache.join("qemu.pid"),
            ovf_vars: cache.join("ovf-vars.fd"),
            ovf_vars_healthy: cache.join("ovf-vars.healthy.fd"),
            overlay: image::overlay_path(cache),
            smbios_uuid: vm::smbios_uuid_from(image_id),
            tpm_state_dir: cache.join("tpm"),
            tpm_sock: cache.join("tpm").join("swtpm-sock"),
            swtpm_pidfile: cache.join("swtpm.pid"),
            swtpm_log: cache.join("swtpm.log"),
        }
    }
}

impl std::fmt::Debug for Ctx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ctx")
            .field("cache", &self.cache)
            .field("base", &self.base.id)
            .finish()
    }
}
