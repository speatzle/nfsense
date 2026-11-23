//! Base-image discovery and the persistent cache (overlay + checkpoint).
//!
//! Layout, under `e2e/.cache/<image-id>/`:
//!
//! | file            | purpose                                                    |
//! |-----------------|------------------------------------------------------------|
//! | `state.qcow2`   | persistent overlay; backs the guest disk; holds the `healthy` internal snapshot |
//! | `base.id`       | marker: id of the base image the overlay was built from   |
//! | `serial.log`    | guest console output (kept on failure)                     |
//! | `qemu.log`      | QEMU stderr (kept on failure)                              |
//! | `qemu.pid`      | pidfile                                                    |
//! | `vm.lock`       | flock serialising all harness processes on this cache      |
//! | `ovf-vars.fd`   | fresh OVMF_VARS copy per spawn                             |
//! | `tpm/`          | swtpm state dir — the guest's TPM 2.0 (persists across runs, like a real TPM) |
//! | `swtpm.pid`     | swtpm pidfile                                              |
//! | `swtpm.log`     | swtpm output                                               |
//!
//! `image-id` is `{stem}-{mtime_ms}-{size}` of the base image, so a new
//! image build moves to a fresh cache (old ones can be pruned by hand)
//! and an unchanged image reuses its checkpoint.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::host;

/// A discovered base (pristine, never written to) image.
#[derive(Debug, Clone)]
pub struct BaseImage {
    pub path: PathBuf,
    /// `{stem}-{mtime_ms}-{size}`; the cache directory key.
    pub id: String,
}

/// Default image location: `<project root>/mkosi/mkosi.output` (the
/// `build-image.sh` output dir). Overridable per invocation.
pub fn default_image_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../mkosi/mkosi.output")
}

/// Find the base image: `dir` may be an explicit .qcow2 file or a
/// directory (then the last .qcow2 in it, sort order); `None` falls back
/// to [`default_image_dir`].
pub fn find_base_image(dir: Option<&Path>) -> Result<BaseImage, String> {
    let path = match dir {
        Some(p) if p.is_file() => p.to_path_buf(),
        Some(d) => host::find_qcow2(d)?,
        None => host::find_qcow2(&default_image_dir())?,
    };
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
        id: format!("{stem}-{mtime_ms}-{}", meta.len()),
        path,
    })
}

pub fn cache_dir(base: &BaseImage) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(".cache")
        .join(&base.id)
}

pub fn overlay_path(cache: &Path) -> PathBuf {
    cache.join("state.qcow2")
}

/// Ensure the persistent overlay exists and matches `base`. Rebuilds it
/// (and only it) when the base image changed or the overlay vanished.
/// Must run while no QEMU is up on it.
pub fn ensure_overlay(base: &BaseImage, cache: &Path) -> Result<PathBuf, String> {
    fs::create_dir_all(cache).map_err(|e| format!("create {}: {e}", cache.display()))?;
    let overlay = overlay_path(cache);
    let marker = cache.join("base.id");

    let current = fs::read_to_string(&marker).ok();
    if overlay.exists() && current.as_deref() == Some(base.id.as_str()) {
        return Ok(overlay);
    }
    // New base (or missing marker): the overlay is stale, rebuild it.
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

/// Does the overlay contain an internal snapshot named `tag`?
/// `qemu-img info` lists them under "Snapshot list:" with the tag in the
/// second column. Must run while no QEMU holds the image open (it does —
/// the harness checks this *before* spawning QEMU).
pub fn overlay_has_snapshot(overlay: &Path, tag: &str) -> bool {
    let Ok(out) = Command::new("qemu-img")
        .args(["info"])
        .arg(overlay)
        .output()
    else {
        return false;
    };
    if !out.status.success() {
        return false;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let Some(start) = text.find("Snapshot list:") else {
        return false;
    };
    text[start..]
        .lines()
        .skip(1) // header: ID TAG ...
        .any(|line| {
            let mut cols = line.split_whitespace();
            cols.next()
                .is_some_and(|id| id.chars().all(|c| c.is_ascii_digit()))
                && cols.next().is_some_and(|t| t == tag)
        })
}
