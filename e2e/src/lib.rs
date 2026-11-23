//! End-to-end test harness for nfsense.
//!
//! Boots the mkosi disk image in QEMU (KVM) and drives the guest through
//! two independent planes:
//!
//!  - **Control plane** — the QEMU Guest Agent over the virtio-serial
//!    chardev unix socket (QGA wire protocol). Independent of the guest's
//!    network stack, so it survives any guest networking, including
//!    configs a test just broke.
//!  - **API plane** — the JSON-RPC API is called from *inside* the guest
//!    via `curl` over loopback (`127.0.0.1:4444`; the server binds
//!    `[::]:4444`). There is no management NIC: a management interface
//!    would pollute network tests (routing, multiwan) and would die
//!    together with the guest's network whenever a test under test
//!    breaks it, hiding the failure from the harness.
//!
//! The VM also gets a stable SMBIOS UUID (→ stable machine-id, which the
//! machine-id-derived `var` GPT partition type requires) and a TPM 2.0
//! from the host's `swtpm` (QEMU `emulator` TPM backend) — without the
//! TPM the in-guest repart fails and `/var` degrades to the root tmpfs,
//! resetting the config at every boot. See `crate::vm` and `crate::tpm`.
//!
//! **Checkpoint/restore model.** Instead of QEMU's `-snapshot` (a RAM
//! overlay discarded on exit), the harness keeps a *persistent* qcow2
//! overlay under `e2e/.cache/<image-id>/` carrying a full internal VM
//! snapshot (`healthy`) — RAM, devices and disk — taken once after a cold
//! boot has settled. Every test starts from `ctx.restore()`, a QMP
//! `snapshot-load` on the running VM (milliseconds, no reboot), so tests
//! are independent of each other and of execution order. Subsequent
//! runs of the harness skip the cold boot entirely: spawn QEMU, load the
//! checkpoint, done. The swtpm state (`cache/tpm/`) is *not* part of the
//! checkpoint: like a real TPM it persists independently across
//! restores, and it is discarded together with the overlay when a
//! restore falls back to a cold boot (a stale TPM would no longer
//! unseal what the checkpointed boot sealed).
//!
//! Protocol notes (QEMU 10.x / current qemu-ga):
//!  - The QMP `guest-*` monitor commands were removed in QEMU 10. Guest
//!    commands must go through the guest agent socket: one JSON object
//!    per line each direction, `guest-`-prefixed commands.
//!  - `guest-exec` takes `path` plus its argv under the key `arg` (not
//!    `arguments`) and returns `pid` as an integer.
//!  - `guest-exec-status` reports `exited: true` plus base64 `out-data` /
//!    `err-data` and the numeric `exitcode` when the process finished.
//!  - `snapshot-save` / `snapshot-load` are *jobs* in QEMU 10: they take
//!    `{job-id, tag, vmstate, devices}` and return immediately; poll
//!    `query-jobs` until the job `concluded` and check its `error`.
//!  - The web API requires a session: `POST /login` first, then
//!    `POST /api` with the `session` cookie. There is no `ping`
//!    JSON-RPC method.
//!
//! Usage: `cargo run -p nfsense-e2e` (scenario, see `main.rs`) or
//! `cargo test -p nfsense-e2e` (test groups). Without `/dev/kvm` the
//! tests skip, keeping `cargo test --workspace` green on CI runners.

pub mod api;
pub mod checks;
pub mod ctx;
pub mod host;
pub mod image;
pub mod qga;
pub mod qmp;
pub mod tpm;
pub mod vm;

/// Block device node name of the guest disk (see `vm::Vm::spawn`). The
/// `vmstate` / `devices` arguments of QMP `snapshot-save` /
/// `snapshot-load` refer to node names, not device aliases.
pub const DISK_NODE: &str = "disk0";

/// Name of the internal VM snapshot kept in the persistent overlay.
pub const CHECKPOINT: &str = "healthy";

/// Where a fully settled cold boot is observed to take, worst case.
pub mod timeouts {
    /// Guest agent up after a cold boot (first boot does machine-id,
    /// firstboot, nfsense-init).
    pub const BOOT: std::time::Duration = std::time::Duration::from_secs(180);
    /// Web API reachable and session login working.
    pub const API: std::time::Duration = std::time::Duration::from_secs(90);
    /// A single in-guest API call (apply runs several `systemctl` jobs).
    pub const API_CALL: std::time::Duration = std::time::Duration::from_secs(120);
    /// Budget for the pristine-image boot job queue to drain (worst case:
    /// wait-online 120 s + chrony-wait 180 s; the gate starts well into
    /// the boot, so 240 s from the gate has margin).
    pub const BOOT_DRAIN: std::time::Duration = std::time::Duration::from_secs(240);
    /// Agent back after a `snapshot-load` / reboot.
    pub const AGENT_RECONNECT: std::time::Duration = std::time::Duration::from_secs(120);
    /// A `systemctl is-active` poll to declare a unit active.
    pub const UNIT: std::time::Duration = std::time::Duration::from_secs(90);
}
