//! E2E scenario driver: boots (or checkpoint-restores) the guest and
//! runs the checks sequentially, each from the `healthy` checkpoint.
//!
//! ```sh
//! cargo run -p nfsense-e2e                   # default image dir
//! cargo run -p nfsense-e2e -- /path/to/img    # explicit dir or .qcow2
//! E2E_IMAGE_DIR=/path/to/img cargo run -p nfsense-e2e
//! ```
//!
//! Exits 0 when all checks pass or when the environment cannot run e2e
//! at all (missing KVM / image / QEMU / OVMF / swtpm — printed as
//! `e2e SKIP`). Exits 1 on a failed check or on a harness/guest failure
//! during init (printed as `e2e FAIL`); on failure the cache dir keeps
//! the serial log, QEMU log and overlay for inspection.

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::{Duration, Instant};

use nfsense_e2e::checks;
use nfsense_e2e::ctx::{Ctx, InitError};

/// One scenario check: a name + a step that starts from the healthy
/// checkpoint.
type Check = (&'static str, Box<dyn FnMut(&mut Ctx) -> Result<(), String>>);

struct TestResults {
    results: Vec<(&'static str, Result<(), String>)>,
}

impl TestResults {
    fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    fn record(&mut self, name: &'static str, res: Result<(), String>) {
        match &res {
            Ok(()) => println!("  PASS {name}"),
            Err(e) => println!("  FAIL {name}: {e}"),
        }
        self.results.push((name, res));
    }

    fn all_ok(&self) -> bool {
        self.results.iter().all(|(_, r)| r.is_ok())
    }

    fn print_summary(&self) {
        println!();
        println!("========================================");
        println!("Results");
        for (name, res) in &self.results {
            let status = if res.is_ok() { "PASS" } else { "FAIL" };
            println!("  {name:<20} {status}");
        }
        println!("  ALL PASSED:         {}", self.all_ok());
        println!("========================================");
    }
}

fn main() -> ExitCode {
    // Image dir: CLI arg wins, then $E2E_IMAGE_DIR, then the default.
    let dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("E2E_IMAGE_DIR").map(PathBuf::from));

    let mut ctx = match Ctx::try_new(dir.as_deref()) {
        Ok(c) => c,
        Err(e) => {
            println!("{e}");
            return match e {
                InitError::Skip(_) => ExitCode::SUCCESS,
                InitError::Fail(_) => ExitCode::FAILURE,
            };
        }
    };
    eprintln!(
        "e2e: image {} — cache {}",
        ctx.base.path.display(),
        ctx.cache.display()
    );

    let mut results = TestResults::new();
    let checks: Vec<Check> = vec![
        (
            "nfsense installed",
            Box::new(|ctx: &mut Ctx| {
                let qga = ctx.agent()?;
                checks::nfsense_installed(qga)
            }),
        ),
        (
            "nfsense service",
            Box::new(|ctx: &mut Ctx| {
                let qga = ctx.agent()?;
                checks::nfsense_service(qga)
            }),
        ),
        (
            "API health",
            Box::new(|ctx: &mut Ctx| {
                let qga = ctx.agent()?;
                checks::api_health(qga)
            }),
        ),
        (
            "Default config",
            Box::new(|ctx: &mut Ctx| {
                let qga = ctx.agent()?;
                checks::default_config(qga)
            }),
        ),
        (
            "NFTables table",
            Box::new(|ctx: &mut Ctx| {
                let qga = ctx.agent()?;
                checks::nftables_table(qga)
            }),
        ),
        (
            "NFTables rules",
            Box::new(|ctx: &mut Ctx| {
                let qga = ctx.agent()?;
                checks::nftables_rules(qga)
            }),
        ),
        (
            "Apply config",
            Box::new(|ctx: &mut Ctx| {
                let qga = ctx.agent()?;
                checks::apply_rule(qga)
            }),
        ),
        // Final, destructive step: kills the shared VM. The real
        // reboot-path check is the `reboot_after_apply` test
        // (apply + `Ctx::reboot` + the persistence assertion in
        // `tests/apply.rs`).
        ("Reboot/power-off", Box::new(power_off)),
    ];

    for (name, mut check) in checks {
        // Every check starts from the healthy checkpoint.
        let res = ctx.restore().and_then(|()| check(&mut ctx));
        results.record(name, res);
    }

    results.print_summary();

    if results.all_ok() {
        return ExitCode::SUCCESS;
    }
    let failed: Vec<&str> = results
        .results
        .iter()
        .filter(|(_, r)| r.is_err())
        .map(|(n, _)| *n)
        .collect();
    eprintln!(
        "e2e failed: {} check(s) — {}",
        results.results.len() - results.results.iter().filter(|(_, r)| r.is_ok()).count(),
        failed.join(", ")
    );
    eprintln!("e2e: artifacts kept in {}", ctx.cache.display());
    // Returning (instead of process::exit) lets the Ctx drop and
    // kills any QEMU that is still alive.
    ExitCode::FAILURE
}

/// Power off the guest via its ACPI/shutdown path and verify QEMU exits
/// with a power-down marker in the serial log.
///
/// This is deliberately a *scenario* step (final step of this binary
/// driver), not a `#[test]`: it kills the shared VM, and the real
/// reboot-path check is the `reboot_after_apply` test.
fn power_off(ctx: &mut Ctx) -> Result<(), String> {
    match ctx.agent().and_then(|q| q.power_off()) {
        Ok(()) => eprintln!("e2e: guest-shutdown accepted"),
        Err(e) => eprintln!("e2e: {e} (expected once the guest goes down)"),
    }

    // QEMU exits on guest power-down by default.
    let exited_ok = ctx
        .vm
        .wait_exited(Instant::now() + Duration::from_secs(90))
        .map_err(|e| format!("wait for power-down: {e}"))?;
    if !exited_ok {
        return Err("QEMU did not exit within 90s after guest-shutdown".into());
    }
    let serial = std::fs::read_to_string(&ctx.vm.paths.serial_log).unwrap_or_default();
    let markers = [
        "Power down",
        "Power Down",
        "Power-Off",
        "Powering off",
        "sleep state S5",
    ];
    if markers.iter().any(|m| serial.contains(m)) {
        Ok(())
    } else {
        Err("serial log has no power-down marker".into())
    }
}
