use validator::Validate;

use super::definitions::config::Config;
use std::error::Error;
use std::fs;

const CURRENT_CONFIG_PATH: &str = "config.json";
const PENDING_CONFIG_PATH: &str = "pending.json";

pub struct ConfigManager {
    current_config: Config,
    pending_config: Config,
}

impl ConfigManager {
    pub fn get_current_config(&self) -> Config {
        self.current_config.clone()
    }

    pub fn get_pending_config(&self) -> Config {
        self.pending_config.clone()
    }

    pub fn apply_pending_changes(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO run Apply functions, revert on failure
        write_config_to_file(CURRENT_CONFIG_PATH, self.pending_config.clone())?;
        // Also revert if config save fails
        self.current_config = self.pending_config.clone();
        Ok(())
    }

    pub fn discard_pending_changes(&mut self) -> Result<(), Box<dyn Error>> {
        self.pending_config = self.current_config.clone();
        Ok(())
    }

    pub fn start_transaction(&mut self) -> Result<ConfigTransaction, Box<dyn Error>> {
        Ok(ConfigTransaction {
            finished: false,
            //guard: guard,
            changes: self.pending_config.clone(),
            config_manager: self,
        })
    }
}

pub struct ConfigTransaction<'a> {
    finished: bool,
    config_manager: &'a mut ConfigManager,
    pub changes: Config,
}

impl<'a> ConfigTransaction<'a> {
    pub fn commit(&mut self) -> Result<(), Box<dyn Error>> {
        let ch = self.changes.clone();
        ch.validate()?;
        self.config_manager.pending_config = ch.clone();
        Ok(())
    }
}

pub fn new_config_manager() -> Result<ConfigManager, Box<dyn Error>> {
    Ok(ConfigManager {
        current_config: read_file_to_config(CURRENT_CONFIG_PATH)?,
        pending_config: read_file_to_config(PENDING_CONFIG_PATH)?,
    })
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
