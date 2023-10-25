use validator::Validate;

use super::definitions::config::Config;
use std::error::Error;
use std::fs;
use std::sync::{Arc, Mutex, MutexGuard};

const CURRENT_CONFIG_PATH: &str = "config.json";
const PENDING_CONFIG_PATH: &str = "pending.json";

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
    pub fn new() -> Result<Self, Box<dyn Error>> {
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

    pub fn apply_pending_changes(&mut self) -> Result<(), Box<dyn Error>> {
        let mut data = self.shared_data.lock().unwrap();
        // TODO run Apply functions
        // TODO Revert on Apply Failure and Return
        write_config_to_file(CURRENT_CONFIG_PATH, data.pending_config.clone())?;
        // TODO revert if config save fails
        // TODO Remove Pending Config File
        data.current_config = data.pending_config.clone();
        Ok(())
    }

    pub fn discard_pending_changes(&mut self) -> Result<(), Box<dyn Error>> {
        let mut data = self.shared_data.lock().unwrap();
        // TODO Remove Pending Config File

        data.pending_config = data.current_config.clone();
        Ok(())
    }

    pub fn start_transaction(&mut self) -> Result<ConfigTransaction, Box<dyn Error>> {
        let data = self.shared_data.lock().unwrap();

        Ok(ConfigTransaction {
            finished: false,
            changes: data.pending_config.clone(),
            shared_data: data,
        })
    }
}

pub struct ConfigTransaction<'a> {
    finished: bool,
    shared_data: MutexGuard<'a, SharedData>,
    pub changes: Config,
}

impl<'a> ConfigTransaction<'a> {
    pub fn commit(mut self) -> Result<(), Box<dyn Error>> {
        let ch = self.changes.clone();
        ch.validate()?;
        self.shared_data.pending_config = ch.clone();
        write_config_to_file(PENDING_CONFIG_PATH, ch.clone())?;
        Ok(())
    }
}

fn read_file_to_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let conf: Config = serde_json::from_str(&data)?;
    if conf.config_version != 1 {
        // return Err("Unsupported config Version");
    }
    Ok(conf)
}

fn write_config_to_file(path: &str, conf: Config) -> Result<(), Box<dyn Error>> {
    let data: String = serde_json::to_string_pretty(&conf)?;
    fs::write(path, data)?;
    Ok(())
}
