//! Host-side prerequisites and cleanup.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

/// Everything the harness needs on the host, resolved once. `Err` is the
/// *skip reason* (missing KVM / QEMU / OVMF) — not a failure.
pub struct Prereqs {
    pub uefi_code: PathBuf,
    pub uefi_vars: PathBuf,
}

pub fn check_prereqs() -> Result<Prereqs, String> {
    if !Command::new("qemu-system-x86_64")
        .arg("--version")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        return Err("qemu-system-x86_64 not found on the host".into());
    }
    if !Path::new("/dev/kvm").exists() {
        return Err("/dev/kvm not available — e2e requires KVM".into());
    }
    let (code, vars) = detect_uefi()?;
    if !Command::new("swtpm")
        .arg("--version")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        return Err(
            "swtpm not found on the host (apt install swtpm) — the guest needs a TPM 2.0 for in-guest repart (persistent /var)"
                .into(),
        );
    }
    Ok(Prereqs {
        uefi_code: code,
        uefi_vars: vars,
    })
}

/// Find a .qcow2 image in `dir` (last in sort order).
pub fn find_qcow2(dir: &Path) -> Result<PathBuf, String> {
    let mut found: Vec<PathBuf> = fs::read_dir(dir)
        .map_err(|e| format!("cannot read {}: {e}", dir.display()))?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "qcow2"))
        .map(|e| e.path())
        .collect();
    found.sort();
    found.pop().ok_or_else(|| {
        format!(
            "no .qcow2 found in {} (build with build-image.sh first?)",
            dir.display()
        )
    })
}

fn detect_uefi() -> Result<(PathBuf, PathBuf), String> {
    for code in [
        "/usr/share/OVMF/OVMF_CODE_4M.fd",
        "/usr/share/edk2/x64/OVMF_CODE.fd",
        "/usr/share/qemu/OVMF_CODE.fd",
    ] {
        if Path::new(code).exists() {
            for vars in [
                "/usr/share/OVMF/OVMF_VARS_4M.fd",
                "/usr/share/edk2/x64/OVMF_VARS.fd",
                "/usr/share/qemu/OVMF_VARS.fd",
            ] {
                if Path::new(vars).exists() {
                    return Ok((code.into(), vars.into()));
                }
            }
        }
    }
    Err("no UEFI firmware found — install edk2-ovmf (Debian) or OVMF (Fedora/Arch)".into())
}

/// Kill a leftover process from a crashed previous run that wrote this
/// pidfile. (Matching by pidfile rather than process name: a
/// `pkill -f` pattern can match the invoking shell's own command line.)
/// `comm` is the expected `/proc/<pid>/comm` (e.g. `qemu-system-x86_64`,
/// `swtpm`) so we never kill an unrelated recycled pid.
pub fn kill_stale_process(pidfile: &Path, comm: &str) {
    let Ok(s) = fs::read_to_string(pidfile) else {
        return;
    };
    let Ok(pid) = s.trim().parse::<i32>() else {
        return;
    };
    if pid <= 1 || !Path::new(&format!("/proc/{pid}")).exists() {
        return;
    }
    let got = fs::read_to_string(format!("/proc/{pid}/comm")).unwrap_or_default();
    if got.trim() != comm {
        return;
    }
    eprintln!("e2e: killing stale {comm} (pid {pid}) left behind by a previous run...");
    let _ = Command::new("kill").args(["-9", &pid.to_string()]).status();
    std::thread::sleep(Duration::from_millis(500));
}

/// Kill a leftover QEMU (see [`kill_stale_process`]).
pub fn kill_stale_qemu(pidfile: &Path) {
    kill_stale_process(pidfile, "qemu-system-x86_64");
}
