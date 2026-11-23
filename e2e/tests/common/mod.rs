//! Shared harness for the e2e test groups.
//!
//! Each group is a plain integration-test binary; every `#[test]` runs
//! through [`with_ctx`], which:
//!
//!  - takes the process-wide `Mutex` (libtest's thread pool would
//!    otherwise race — and the QGA chardev accepts one host connection
//!    at a time, so serialisation is mandatory, not an optimisation);
//!  - creates the shared `Ctx` on first use, *skipping* (with a note,
//!    not a failure) when the environment cannot run e2e at all —
//!    missing KVM / QEMU / OVMF / swtpm / image (`InitError::Skip`) —
//!    while *failing* when the prerequisites were present but the
//!    harness or the guest misbehaved (`InitError::Fail`); that
//!    distinction keeps `cargo test --workspace` green on KVM-less CI
//!    runners without letting a broken image pass as "skipped";
//!  - caches a failed init for the whole binary, so it is not retried
//!    per test (each retry could re-wait on the run lock for minutes);
//!  - respawns QEMU if the VM is down (every previous test shut it
//!    down) and restores the `healthy` checkpoint, so every test
//!    starts from the same pristine state regardless of what its
//!    neighbours did (and of which binary ran first);
//!  - shuts the VM down again when the test finishes (success,
//!    failure, or panic) — a failing libtest binary exits via
//!    `process::exit`, which skips Rust static destructors, so we
//!    cannot rely on `Drop` for cleanup.
//!
//! Across *binaries* (and across repeated invocations), the same VM
//! cache is serialised by the `flock` on `cache/vm.lock` inside
//! `Ctx::try_new`.

#![allow(dead_code)]

use std::sync::{Mutex, OnceLock};

use nfsense_e2e::ctx::{Ctx, InitError};
use nfsense_e2e::image::default_image_dir;

static CTX: OnceLock<Mutex<Option<Ctx>>> = OnceLock::new();
static INIT_FAIL: OnceLock<String> = OnceLock::new();

/// Base-image directory: `$E2E_IMAGE_DIR` if set, else the default
/// `mkosi/mkosi.output`.
fn image_dir() -> std::path::PathBuf {
    std::env::var_os("E2E_IMAGE_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(default_image_dir)
}

/// Shuts the VM down whenever it is dropped — around the test body, so
/// it fires on success, failure, *and* panic.
struct ShutdownGuard<'a> {
    ctx: &'a mut Ctx,
}

impl Drop for ShutdownGuard<'_> {
    fn drop(&mut self) {
        eprintln!("e2e: shutting VM down after test");
        self.ctx.shutdown();
    }
}

/// Run `f` against a healthy, checkpoint-restored guest.
pub fn with_ctx(f: impl FnOnce(&mut Ctx) -> Result<(), String>) {
    let m = CTX.get_or_init(|| Mutex::new(None));
    let mut g = m.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let ctx = match g.as_mut() {
        Some(c) => c,
        None => {
            if let Some(fail) = INIT_FAIL.get() {
                panic!("e2e FAIL: {fail}");
            }
            match Ctx::try_new(Some(&image_dir())) {
                Ok(c) => g.insert(c),
                Err(InitError::Skip(r)) => {
                    eprintln!("e2e SKIP: {r}");
                    return; // skip, do not fail
                }
                Err(InitError::Fail(r)) => {
                    let _ = INIT_FAIL.set(r.clone());
                    panic!("e2e FAIL: {r}");
                }
            }
        }
    };

    if let Err(e) = ctx.restore() {
        ctx.shutdown();
        panic!("e2e: restore to checkpoint failed: {e}");
    }

    let mut guard = ShutdownGuard { ctx };
    let ctx = &mut guard.ctx;
    if let Err(e) = f(ctx) {
        panic!("{e}");
    }
    // The guard drops here (or on panic), killing QEMU either way.
}
