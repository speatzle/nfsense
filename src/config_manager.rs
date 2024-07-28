use super::definitions::config::Config;
use garde::Validate;
use pwhash::sha512_crypt;
use serde::Serialize;
use std::fs;
use std::sync::{Arc, Mutex, MutexGuard};
use thiserror::Error;
use tracing::{error, info};

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Json Error")]
    SerdeError(#[from] serde_json::Error),

    #[error("Validation Error")]
    ValidatonError(#[from] garde::Report),

    #[error("Hash Error")]
    HashError(#[from] pwhash::error::Error),

    #[error("Unsupported config version")]
    UnsupportedVersionError,

    #[error("Apply Error")]
    ApplyError(#[from] super::apply::ApplyError),

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

pub const CURRENT_CONFIG_PATH: &str = "config.json";
pub const PENDING_CONFIG_PATH: &str = "pending.json";

static APPLY_FUNCTIONS: &'static [fn(
    pending_config: Config,
    current_config: Config,
) -> Result<(), super::apply::ApplyError>] = &[
    super::apply::networkd::apply_networkd,
    super::apply::nftables::apply_nftables,
    super::apply::kea::apply_kea,
    super::apply::chrony::apply_chrony,
    super::apply::unbound::apply_unbound,
];

#[derive(Clone)]
pub struct ConfigManager {
    shared_data: Arc<Mutex<SharedData>>,
}

pub struct ConfigTransaction<'a> {
    shared_data: MutexGuard<'a, SharedData>,
    pub config: Config,
}

struct SharedData {
    current_config: Config,
    pending_config: Config,
    changelog: Vec<Change>,
}

#[derive(Clone, Serialize)]
pub struct Change {
    pub action: ChangeAction,
    pub path: &'static str,
    pub id: String,
}

#[derive(Clone, Serialize)]
pub enum ChangeAction {
    Create,
    Update,
    Delete,
}

// Note, using unwarp on a mutex lock is ok since that only errors with mutex poisoning

impl ConfigManager {
    pub fn new() -> Result<Self, ConfigError> {
        Ok(Self {
            shared_data: Arc::new(Mutex::new(SharedData {
                current_config: read_file_to_config(CURRENT_CONFIG_PATH)?,
                // TODO Dont Fail if pending config is missing, use current instead
                pending_config: read_file_to_config(PENDING_CONFIG_PATH)?,
                // TODO Figure out how to restore changes
                changelog: Vec::new(),
            })),
        })
    }

    pub fn get_current_config(&self) -> Config {
        self.shared_data.lock().unwrap().current_config.clone()
    }

    pub fn get_pending_config(&self) -> Config {
        self.shared_data.lock().unwrap().pending_config.clone()
    }

    pub fn get_pending_changelog(&self) -> Vec<Change> {
        self.shared_data.lock().unwrap().changelog.clone()
    }

    pub fn apply_pending_changes(&mut self) -> Result<(), ConfigError> {
        let mut data = self.shared_data.lock().unwrap();

        // TODO Improve Error Handling
        for apply_function in APPLY_FUNCTIONS {
            match (apply_function)(data.pending_config.clone(), data.current_config.clone()) {
                Ok(_) => info!("Applied"),
                Err(e) => {
                    error!("Applying function, Reverting to current config...");

                    for apply_function in APPLY_FUNCTIONS {
                        match (apply_function)(
                            // These are swapped for revert
                            data.current_config.clone(),
                            data.pending_config.clone(),
                        ) {
                            Ok(_) => info!("Applied"),
                            Err(e) => {
                                error!("Reverting failed, giving up.");
                                return Err(ConfigError::ApplyError(e));
                            }
                        }
                    }

                    info!("Revert Done.");
                    return Err(ConfigError::ApplyError(e));
                }
            }
        }

        info!("Apply Done.");

        write_config_to_file(CURRENT_CONFIG_PATH, data.pending_config.clone())?;
        // TODO revert if config save fails
        // TODO Remove Pending Config File
        data.current_config = data.pending_config.clone();
        data.changelog = Vec::new();
        Ok(())
    }

    pub fn discard_pending_changes(&mut self) -> Result<(), ConfigError> {
        let mut data = self.shared_data.lock().unwrap();
        // TODO Remove Pending Config File

        data.pending_config = data.current_config.clone();
        data.changelog = Vec::new();
        Ok(())
    }

    pub fn start_transaction(&mut self) -> ConfigTransaction {
        let data = self.shared_data.lock().unwrap();

        ConfigTransaction {
            config: data.pending_config.clone(),
            shared_data: data,
        }
    }
}

impl<'a> ConfigTransaction<'a> {
    pub fn commit(mut self, change: Change) -> Result<(), ConfigError> {
        let ch = self.config.clone();
        ch.validate()?;
        self.shared_data.pending_config = ch.clone();
        self.shared_data.changelog.push(change);
        write_config_to_file(PENDING_CONFIG_PATH, ch.clone())?;
        Ok(())
    }

    pub fn revert(self) {}
}

fn read_file_to_config(path: &str) -> Result<Config, ConfigError> {
    let data = fs::read_to_string(path)?;
    let conf: Config = serde_json::from_str(&data)?;
    if conf.config_version != 1 {
        return Err(ConfigError::UnsupportedVersionError);
    }
    conf.validate()?;
    Ok(conf)
}

fn write_config_to_file(path: &str, conf: Config) -> Result<(), ConfigError> {
    let data: String = serde_json::to_string_pretty(&conf)?;
    fs::write(path, data)?;
    Ok(())
}

pub fn generate_default_config(path: &str) -> Result<(), ConfigError> {
    let mut conf = Config::default();
    let hash = sha512_crypt::hash("nfsense")?;
    conf.config_version = 1;
    conf.system.users.push(crate::definitions::system::User {
        name: "admin".to_string(),
        comment: "Default Admin".to_string(),
        hash: hash,
    });
    write_config_to_file(path, conf)
}
