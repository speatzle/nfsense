use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::host;

#[derive(Debug, Clone)]
pub struct BaseImage {
    pub path: PathBuf,
    pub id: String,
}

pub fn default_image_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../mkosi/mkosi.output")
}

pub fn find_base_image(dir: Option<&Path>) -> Result<BaseImage, String> {
    let path = match dir {
        Some(p) if p.is_file() => p.to_path_buf(),
        Some(d) => host::find_qcow2(d)?,
        None => host::find_qcow2(&default_image_dir())?,
    };
    // qemu-img resolves a relative backing path against the overlay's
    // directory, not the cwd.
    let path = path
        .canonicalize()
        .map_err(|e| format!("{}: {e}", path.display()))?;
    let meta = fs::metadata(&path).map_err(|e| format!("stat {path:?}: {e}"))?;
    let mtime_ms = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let stem = path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "image".into());
    Ok(BaseImage {
        id: format!(
            "{stem}-{mtime_ms}-{}-{}",
            meta.len(),
            crate::vm::DEVICE_LAYOUT
        ),
        path,
    })
}

// The cache holds unix sockets, whose paths must stay below 108 bytes: set
// `E2E_CACHE_DIR` to a short path when the checkout is deep.
pub fn cache_dir(base: &BaseImage) -> PathBuf {
    let root = std::env::var_os("E2E_CACHE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(".cache"));
    root.join(&base.id)
}

pub const LONGEST_SOCKET: &str = "tpm/swtpm-sock";

// Fail early instead of with a socket that never appears: `sun_path` is 108
// bytes including the NUL.
pub fn check_socket_path_len(cache: &Path) -> Result<(), String> {
    let len = cache.join(LONGEST_SOCKET).as_os_str().len();
    if len > 107 {
        return Err(format!(
            "cache dir {} is too deep: socket paths would be {len} bytes (limit 107); set E2E_CACHE_DIR to a shorter path",
            cache.display()
        ));
    }
    Ok(())
}

pub fn overlay_path(cache: &Path) -> PathBuf {
    cache.join("state.qcow2")
}

/// Must run while no QEMU is up on the overlay.
pub fn ensure_overlay(base: &BaseImage, cache: &Path) -> Result<PathBuf, String> {
    fs::create_dir_all(cache).map_err(|e| format!("create {}: {e}", cache.display()))?;
    let overlay = overlay_path(cache);
    let marker = cache.join("base.id");

    let current = fs::read_to_string(&marker).ok();
    if overlay.exists() && current.as_deref() == Some(base.id.as_str()) {
        return Ok(overlay);
    }
    if overlay.exists() {
        eprintln!(
            "e2e: rebuilding stale overlay {} (base image changed)",
            overlay.display()
        );
    }
    fs::remove_file(&overlay).ok();
    let out = Command::new("qemu-img")
        .args(["create", "-f", "qcow2", "-F", "qcow2", "-b"])
        .arg(&base.path)
        .arg(&overlay)
        .output()
        .map_err(|e| format!("qemu-img create: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "qemu-img create failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    fs::write(&marker, &base.id).map_err(|e| e.to_string())?;
    Ok(overlay)
}

// Host-side via `qemu-img info`: QEMU 10 has no `query-snapshots` and
// `query-block` does not list internal snapshots.
pub fn overlay_has_snapshot(overlay: &Path, tag: &str) -> Result<bool, String> {
    let out = Command::new("qemu-img")
        .args(["info", "-U", "--output=json"])
        .arg(overlay)
        .output()
        .map_err(|e| format!("qemu-img info: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "qemu-img info {} failed: {}",
            overlay.display(),
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    let info: serde_json::Value = serde_json::from_slice(&out.stdout)
        .map_err(|e| format!("qemu-img info output: {e}"))?;
    Ok(info["snapshots"]
        .as_array()
        .is_some_and(|s| s.iter().any(|s| s["name"] == tag)))
}
