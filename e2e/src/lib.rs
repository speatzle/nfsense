pub mod api;
pub mod checks;
pub mod ctx;
pub mod host;
pub mod image;
pub mod qga;
pub mod qmp;
pub mod serial;
pub mod tpm;
pub mod vm;

pub const DISK_NODE: &str = "disk0";

pub const CHECKPOINT: &str = "healthy";

pub mod timeouts {
    // First boot also runs machine-id setup, firstboot and nfsense-init.
    pub const BOOT: std::time::Duration = std::time::Duration::from_secs(180);
    pub const API: std::time::Duration = std::time::Duration::from_secs(90);
    // Apply runs several `systemctl` jobs.
    pub const API_CALL: std::time::Duration = std::time::Duration::from_secs(120);
    // Worst case wait-online 120 s + chrony-wait 180 s; the gate starts well
    // into the boot, so 240 s from the gate has margin.
    pub const BOOT_DRAIN: std::time::Duration = std::time::Duration::from_secs(240);
    // A full reboot (shutdown, firmware, kernel, initrd, userspace until the
    // guest agent is back) can exceed 2 minutes on a loaded host.
    pub const AGENT_RECONNECT: std::time::Duration = std::time::Duration::from_secs(240);
    pub const UNIT: std::time::Duration = std::time::Duration::from_secs(90);
}
