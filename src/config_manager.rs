use validator::Validate;

use super::definitions::config::Config;
use std::fs;
use std::sync::{Arc, Mutex, MutexGuard};

use pwhash::sha512_crypt;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Json Error")]
    SerdeError(#[from] serde_json::Error),

    #[error("Validation Error")]
    ValidatonError(#[from] validator::ValidationErrors),

    #[error("Hash Error")]
    HashError(#[from] pwhash::error::Error),

    #[error("Unsupported config version")]
    UnsupportedVersionError,

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

pub const CURRENT_CONFIG_PATH: &str = "config.json";
pub const PENDING_CONFIG_PATH: &str = "pending.json";

#[derive(Clone)]
pub struct ConfigManager {
    shared_data: Arc<Mutex<SharedData>>,
}

struct SharedData {
    current_config: Config,
    pending_config: Config,
}

// Note, using unwarp on a mutex lock is ok since that only errors with mutex poisoning

impl ConfigManager {
    pub fn new() -> Result<Self, ConfigError> {
        Ok(Self {
            shared_data: Arc::new(Mutex::new(SharedData {
                current_config: read_file_to_config(CURRENT_CONFIG_PATH)?,
                // TODO Dont Fail if pending config is missing, use current instead
                pending_config: read_file_to_config(PENDING_CONFIG_PATH)?,
            })),
        })
    }

    pub fn get_current_config(&self) -> Config {
        self.shared_data.lock().unwrap().current_config.clone()
    }

    pub fn get_pending_config(&self) -> Config {
        self.shared_data.lock().unwrap().pending_config.clone()
    }

    pub fn apply_pending_changes(&mut self) -> Result<(), ConfigError> {
        let mut data = self.shared_data.lock().unwrap();
        // TODO run Apply functions
        // TODO Revert on Apply Failure and Return
        write_config_to_file(CURRENT_CONFIG_PATH, data.pending_config.clone())?;
        // TODO revert if config save fails
        // TODO Remove Pending Config File
        data.current_config = data.pending_config.clone();
        Ok(())
    }

    pub fn discard_pending_changes(&mut self) -> Result<(), ConfigError> {
        let mut data = self.shared_data.lock().unwrap();
        // TODO Remove Pending Config File

        data.pending_config = data.current_config.clone();
        Ok(())
    }

    pub fn start_transaction(&mut self) -> ConfigTransaction {
        let data = self.shared_data.lock().unwrap();

        ConfigTransaction {
            finished: false,
            changes: data.pending_config.clone(),
            shared_data: data,
        }
    }
}

pub struct ConfigTransaction<'a> {
    finished: bool,
    shared_data: MutexGuard<'a, SharedData>,
    pub changes: Config,
}

impl<'a> ConfigTransaction<'a> {
    pub fn commit(mut self) -> Result<(), ConfigError> {
        let ch = self.changes.clone();
        ch.validate()?;
        self.shared_data.pending_config = ch.clone();
        write_config_to_file(PENDING_CONFIG_PATH, ch.clone())?;
        Ok(())
    }

    pub fn revert(mut self) {}
}

fn read_file_to_config(path: &str) -> Result<Config, ConfigError> {
    let data = fs::read_to_string(path)?;
    let conf: Config = serde_json::from_str(&data)?;
    if conf.config_version != 1 {
        return Err(ConfigError::UnsupportedVersionError);
    }
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
    conf.system.users.insert(
        "admin".to_string(),
        crate::definitions::system::User {
            comment: "Default Admin".to_string(),
            hash: hash,
        },
    );
    write_config_to_file(path, conf)
}
