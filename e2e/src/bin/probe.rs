use std::time::{Duration, Instant};

use nfsense_e2e::ctx::{Ctx, InitError};

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let (reboot, argv) = match argv.first().map(String::as_str) {
        Some("reboot") if argv.len() > 1 => (true, argv[1..].to_vec()),
        _ => (false, argv),
    };
    if argv.is_empty() {
        eprintln!("usage: nfsense-e2e-probe [reboot] <command> [args...]");
        std::process::exit(2);
    }

    let dir = std::env::var_os("E2E_IMAGE_DIR").map(std::path::PathBuf::from);
    let mut ctx = match Ctx::try_new(dir.as_deref()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            match e {
                InitError::Skip(_) => return,
                InitError::Fail(_) => std::process::exit(1),
            }
        }
    };
    if let Err(e) = ctx.restore() {
        eprintln!("e2e: restore failed: {e}");
        ctx.shutdown();
        std::process::exit(1);
    }

    let (path, rest) = argv.split_first().expect("non-empty");
    let args: Vec<&str> = rest.iter().map(|s| s.as_str()).collect();

    let mut fail = false;
    let mut code = 0;
    if let Some(tag) = reboot.then_some("pre") {
        match run(&mut ctx, path, &args) {
            Ok((exit, out, err)) => {
                code = exit;
                print!("{tag}:\n{out}");
                if !err.is_empty() {
                    eprintln!("--- stderr ---\n{err}");
                }
            }
            Err(e) => {
                eprintln!("{tag}: {e}");
                fail = true;
            }
        }
    }

    if reboot {
        if let Err(e) = ctx.reboot() {
            eprintln!("e2e: reboot failed: {e}");
            ctx.shutdown();
            std::process::exit(1);
        }
        match run(&mut ctx, path, &args) {
            Ok((exit, out, err)) => {
                code = exit;
                println!("post:\n{out}");
                if !err.is_empty() {
                    eprintln!("--- stderr ---\n{err}");
                }
            }
            Err(e) => {
                eprintln!("post: {e}");
                fail = true;
            }
        }
    } else {
        match run(&mut ctx, path, &args) {
            Ok((exit, out, err)) => {
                code = exit;
                if !out.is_empty() {
                    println!("{out}");
                }
                if !err.is_empty() {
                    eprintln!("--- stderr ---\n{err}");
                }
            }
            Err(e) => {
                eprintln!("e2e: probe failed: {e}");
                fail = true;
            }
        }
    }

    ctx.shutdown();
    if fail {
        std::process::exit(1);
    }
    // -1: killed by a signal, no exit code.
    std::process::exit(if code < 0 { 1 } else { code.min(255) as i32 });
}

fn run(ctx: &mut Ctx, path: &str, args: &[&str]) -> Result<(i64, String, String), String> {
    let started = Instant::now();
    let (exit, out, err) = ctx
        .agent()
        .and_then(|q| q.exec(path, args, Duration::from_secs(180)))
        .map_err(|e| format!("probe failed after {:?}: {e}", started.elapsed()))?;
    eprintln!("probe {path} exited with {exit} ({:?})", started.elapsed());
    Ok((exit, out, err))
}
