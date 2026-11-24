use std::path::PathBuf;
use std::process::ExitCode;
use std::time::{Duration, Instant};

use nfsense_e2e::checks;
use nfsense_e2e::ctx::{Ctx, InitError};

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
        // Last: it kills the shared VM. The reboot path is covered by the
        // `reboot_after_apply` test.
        ("Reboot/power-off", Box::new(power_off)),
    ];

    for (name, mut check) in checks {
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
    // Returning instead of `process::exit` lets the Ctx drop and kill any
    // QEMU that is still alive.
    ExitCode::FAILURE
}

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
