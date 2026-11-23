//! The harness context: one VM + its QMP/QGA handles + the cache + the
//! run lock.
//!
//! [`Ctx::try_new`] is the single entry point used by both the binary
//! scenario driver and the `#[test]` groups:
//!
//! 1. resolve host prerequisites (KVM, QEMU, OVMF) — failure is a
//!    *skip* reason, not an error;
//! 2. find the base image and its persistent cache dir;
//! 3. take the exclusive run lock (flock on `cache/vm.lock`);
//! 4. ensure the overlay for this base image;
//! 5. if the overlay already carries the `healthy` checkpoint: spawn
//!    QEMU and `snapshot-load` it (seconds); otherwise cold boot, wait
//!    for a fully settled system, and `snapshot-save` the checkpoint;
//! 6. hand back a context whose guest is at the healthy checkpoint.
//!
//! [`Ctx::restore`] then moves the (still running) VM back to that
//! checkpoint in milliseconds — that is what makes the individual tests
//! independent of each other and of execution order.

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

/// Why [`Ctx::try_new`] failed.
///
/// `Skip` — the host cannot run e2e at all (missing KVM / QEMU / OVMF /
/// swtpm / base image). Tests report a note and pass: that is what
/// keeps the suite green on KVM-less CI runners.
///
/// `Fail` — the prerequisites were present but the harness or the guest
/// misbehaved (boot gate, checkpoint save/restore, lock timeout). That
/// is a test failure, not a skip: a broken image must not pass as
/// "skipped".
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
    /// Run lock: kernel-freed when the process dies, so it can never be
    /// stale. Held for the whole lifetime of the Ctx.
    _lock: fs::File,
}

impl Ctx {
    /// Boot (or checkpoint-restore) a healthy guest. `InitError::Skip`
    /// is the skip reason (no KVM / no image / ...); `InitError::Fail`
    /// is a harness/guest failure.
    pub fn try_new(dir: Option<&Path>) -> Result<Ctx, InitError> {
        let prereqs = host::check_prereqs().map_err(InitError::Skip)?;
        let base = image::find_base_image(dir).map_err(InitError::Skip)?;
        let cache = image::cache_dir(&base);
        fs::create_dir_all(&cache)
            .map_err(|e| InitError::Fail(format!("create {}: {e}", cache.display())))?;

        let lock = acquire_lock(&cache).map_err(InitError::Fail)?;
        host::kill_stale_qemu(&cache.join("qemu.pid"));

        let paths = Paths::for_cache(&cache, &prereqs.uefi_code, &base.id);

        let overlay = image::ensure_overlay(&base, &cache).map_err(InitError::Fail)?;
        let has_cp = image::overlay_has_snapshot(&overlay, CHECKPOINT);

        if has_cp {
            eprintln!(
                "e2e: checkpoint '{}' found in {} — fast restore path",
                CHECKPOINT,
                overlay.display()
            );
            if let Ok(ctx) = Self::spawn_and_load(
                &prereqs,
                &base,
                &cache,
                &paths,
                lock.try_clone()
                    .map_err(|e| InitError::Fail(e.to_string()))?,
            ) {
                return Ok(ctx);
            }
            eprintln!("e2e: checkpoint restore failed — falling back to a cold boot");
            fs::remove_file(&overlay).ok();
            // The swtpm state is NOT part of the QEMU snapshot: discard
            // it with the overlay so the cold boot gets a consistent
            // TPM identity (a stale TPM would no longer unseal what the
            // checkpointed boot sealed).
            let _ = fs::remove_dir_all(&paths.tpm_state_dir);
        }

        let overlay = image::ensure_overlay(&base, &cache).map_err(InitError::Fail)?;
        Self::cold_boot(&prereqs, &base, &cache, &paths, &overlay, lock).map_err(InitError::Fail)
    }

    /// Move the running VM back to the `healthy` checkpoint and make
    /// sure the guest agent is reachable. Respawns QEMU first if it
    /// died (a previous test's shutdown, a crash, ...).
    pub fn restore(&mut self) -> Result<(), String> {
        if !self.vm.alive() {
            eprintln!("e2e: VM is down — respawning and restoring checkpoint");
            self.respawn()?;
        }
        self.qmp.restore_checkpoint().map_err(|e| {
            format!(
                "snapshot-load failed: {e}\n--- qemu log tail ---\n{}",
                vm::tail(&self.vm.paths.qemu_log, 1500)
            )
        })?;
        self.agent().map(|_| ())
    }

    /// Reboot the guest for real, the way the product will: an
    /// in-guest `systemctl reboot` (failover and sysupdate reboots are
    /// guest-initiated, not power cuts). PID1 runs the full shutdown —
    /// clean unmount of `/var`, service stops — then reboots the machine
    /// in place: QEMU stays up, OVMF boots again, and the QGA channel
    /// re-pairs on boot.
    ///
    /// Completion is detected by a new kernel boot id
    /// (`/proc/sys/kernel/random/boot_id` changes on every boot) — a
    /// ping on the old connection is *not* sufficient: qemu-ga can
    /// still be alive during the shutdown window, which would make the
    /// harness mistake "guest still up, about to go down" for
    /// "rebooted".
    pub fn reboot(&mut self) -> Result<(), String> {
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
            match Qga::connect(&sock).and_then(|mut q| {
                q.exec_ok(
                    "cat",
                    &["/proc/sys/kernel/random/boot_id"],
                    Duration::from_secs(10),
                )
                .map(|id| (q, id.trim().to_string()))
            }) {
                Ok((q, id)) if id != old_boot_id => {
                    eprintln!("e2e: guest rebooted (new boot id {id})");
                    self.qga = Some(q);
                    return Ok(());
                }
                _ => {}
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

    /// Simulate a power loss: QMP `system_reset` — a sudden cold reset
    /// with no unmount. Stronger than `reboot()` for persistence checks
    /// (crash resilience, ext4 journal recovery), but not the path the
    /// product uses. A `sync` is issued first so deliberate writes
    /// (e.g. the config just written by `config.pending.apply`) reach
    /// disk before the reset — without it they would still sit in the
    /// guest page cache and be lost.
    pub fn hard_reset(&mut self) -> Result<(), String> {
        eprintln!("e2e: simulating power loss (system_reset)...");
        let qga = self.agent()?;
        qga.exec_ok("sync", &[], Duration::from_secs(30))?;
        self.qmp.cmd("system_reset", None)?;
        self.qga = None;
        self.agent()?;
        Ok(())
    }

    /// A live, ping-able guest agent; reconnects through QEMU if the
    /// channel dropped (post-restore, post-reboot).
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

    /// Spawn QEMU on the persistent overlay and restore the checkpoint
    /// into it.
    fn spawn_and_load(
        prereqs: &host::Prereqs,
        base: &BaseImage,
        cache: &Path,
        paths: &Paths,
        lock: fs::File,
    ) -> Result<Ctx, String> {
        prepare_spawn(paths, &prereqs.uefi_vars)?;
        let mut vm = Vm::spawn(paths.clone(), &prereqs.uefi_code, &paths.ovf_vars)?;
        let mut qmp = Qmp::connect(&paths.qmp_sock, Instant::now() + Duration::from_secs(30))?;
        qmp.restore_checkpoint()?;
        let qga = Qga::wait_ready(&paths.qga_sock, Instant::now() + AGENT_RECONNECT, || {
            vm.alive()
        })
        .map_err(|e| {
            format!(
                "agent not reachable after snapshot-load: {e}\n--- serial tail ---\n{}",
                vm::tail(&paths.serial_log, 2000)
            )
        })?;
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

    /// Cold boot: full boot, wait for a settled system, save the
    /// checkpoint.
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
        let mut vm = Vm::spawn(paths.clone(), &prereqs.uefi_code, &paths.ovf_vars)?;
        let mut qmp = Qmp::connect(&paths.qmp_sock, Instant::now() + Duration::from_secs(30))?;

        let mut qga = Qga::wait_ready(
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
        checks::wait_unit_active(&mut qga, "nfsense")?;
        checks::wait_boot_jobs_drained(&mut qga)?;
        checks::api_ready(&mut qga, || vm.alive())?;

        // Flush the guest's page cache so the checkpoint captures a
        // consistent on-disk state (first-boot config writes included).
        qga.exec_ok("sync", &[], Duration::from_secs(30))?;

        eprintln!("e2e: saving checkpoint '{CHECKPOINT}'...");
        qmp.snapshot_save(DISK_NODE, CHECKPOINT)?;
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

    /// Tear the VM down between tests (QMP quit, SIGKILL fallback).
    /// The overlay and checkpoint are untouched on disk, so the next
    /// test restores from them via [`Ctx::restore`].
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

    /// Re-spawn QEMU on the same overlay after the VM died.
    pub fn respawn(&mut self) -> Result<(), String> {
        // Reap the corpse.
        self.vm.reap();

        prepare_spawn(&self.vm.paths, &self.uefi_vars)?;
        self.vm = Vm::spawn(
            self.vm.paths.clone(),
            &self.uefi_code,
            &self.vm.paths.ovf_vars,
        )?;
        self.qmp = Qmp::connect(
            &self.vm.paths.qmp_sock,
            Instant::now() + Duration::from_secs(30),
        )?;
        Ok(())
    }
}

/// Fresh OVMF vars for this spawn + clean socket files.
fn prepare_spawn(paths: &Paths, uefi_vars: &Path) -> Result<(), String> {
    fs::copy(uefi_vars, &paths.ovf_vars).map_err(|e| {
        format!(
            "copy OVMF_VARS template to {}: {e}",
            paths.ovf_vars.display()
        )
    })?;
    fs::remove_file(&paths.qga_sock).ok();
    fs::remove_file(&paths.qmp_sock).ok();
    fs::File::create(&paths.serial_log).map_err(|e| e.to_string())?;
    Ok(())
}

/// Exclusive flock on `cache/vm.lock`, retried while another harness
/// process legitimately holds it (a cold boot + checks can take minutes).
/// The kernel releases the lock when the holder dies, so this can only
/// block on a *live* harness.
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
    /// Cache-dir-local paths for one cache.
    pub fn for_cache(cache: &Path, uefi_code: &Path, image_id: &str) -> Self {
        Self {
            qga_sock: cache.join("ga.sock"),
            qmp_sock: cache.join("mp.sock"),
            serial_log: cache.join("serial.log"),
            qemu_log: cache.join("qemu.log"),
            pidfile: cache.join("qemu.pid"),
            ovf_vars: cache.join("ovf-vars.fd"),
            ovf_code: uefi_code.to_path_buf(),
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
