use super::definitions::config::Config;
use garde::Validate;
use pwhash::sha512_crypt;
use serde::Serialize;
use std::fs;
use std::sync::{Arc, Mutex, MutexGuard};
use structdb_core::change::Change;
use structdb_core::Diffable;
use thiserror::Error;
use time::OffsetDateTime;
use tracing::{error, info};

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Json Error")]
    SerdeError(#[from] serde_json::Error),

    #[error("Validation Error")]
    ValidatonError(#[from] garde::Report),

    #[error("Reference validation error: {0:?}")]
    ReferenceValidationError(Vec<structdb_core::ValidationError>),

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

pub const CURRENT_CONFIG_PATH: &str = "/var/lib/nfsense/config.json";
pub const PENDING_CONFIG_PATH: &str = "/var/lib/nfsense/pending.json";

static GENERATE_FUNCTIONS: &'static [fn(
    apply: bool,
    pending_config: Config,
    current_config: Config,
) -> Result<(), super::apply::ApplyError>] = &[
    super::apply::networkd::generate_networkd,
    super::apply::nftables::generate_nftables,
    super::apply::kea::generate_kea,
    super::apply::chrony::generate_chrony,
    super::apply::unbound::generate_unbound,
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
    changelog: Vec<ChangeSet>,
    in_memory: bool, // Used to disable file I/O for testing
}

#[derive(Debug, Clone, Serialize)]
pub struct ChangeSet {
    pub user: String,
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
    pub changes: Vec<Change>,
}

// Note, using unwarp on a mutex lock is ok since that only errors with mutex poisoning

impl ConfigManager {
    pub fn new() -> Result<Self, ConfigError> {
        let current_config = read_file_to_config(CURRENT_CONFIG_PATH)?;

        let pending_config = read_file_to_config(PENDING_CONFIG_PATH)?;

        let changelog = {
            let changes = pending_config.diff(&current_config, &mut vec![]);
            if !changes.is_empty() {
                info!(
                    "Reconstructed {} unapplied change(s) from saved config",
                    changes.len()
                );
                vec![ChangeSet {
                    user: "unknown".to_string(),
                    timestamp: OffsetDateTime::now_utc(),
                    changes,
                }]
            } else {
                Vec::new()
            }
        };

        Ok(Self {
            shared_data: Arc::new(Mutex::new(SharedData {
                current_config,
                pending_config,
                changelog,
                in_memory: false,
            })),
        })
    }

    pub fn get_current_config(&self) -> Config {
        self.shared_data.lock().unwrap().current_config.clone()
    }

    pub fn get_pending_config(&self) -> Config {
        self.shared_data.lock().unwrap().pending_config.clone()
    }

    pub fn get_pending_changelog(&self) -> Vec<ChangeSet> {
        self.shared_data.lock().unwrap().changelog.clone()
    }

    pub fn add_to_changelog(&mut self, change_set: ChangeSet) {
        self.shared_data.lock().unwrap().changelog.push(change_set);
    }

    pub fn set_pending_config(&mut self, config: Config, user: String) -> Result<(), ConfigError> {
        validate_config(&config)?;
        let mut data = self.shared_data.lock().unwrap();

        let changes = config.diff(&data.current_config, &mut vec![]);

        data.pending_config = config.clone();

        if !data.in_memory {
            write_config_to_file(PENDING_CONFIG_PATH, config)?;
        }

        if !changes.is_empty() {
            data.changelog = vec![ChangeSet {
                user,
                timestamp: OffsetDateTime::now_utc(),
                changes,
            }];
        } else {
            data.changelog = Vec::new();
        }

        Ok(())
    }

    pub fn apply_pending_changes(&mut self) -> Result<(), ConfigError> {
        let mut data = self.shared_data.lock().unwrap();

        // TODO Improve Error Handling
        for apply_function in GENERATE_FUNCTIONS {
            match (apply_function)(
                true,
                data.pending_config.clone(),
                data.current_config.clone(),
            ) {
                Ok(_) => info!("Applied"),
                Err(e) => {
                    error!("Ran into an error while applying, Reverting to current config... Error: {}", e);

                    for apply_function in GENERATE_FUNCTIONS {
                        match (apply_function)(
                            true,
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

        if !data.in_memory {
            write_config_to_file(CURRENT_CONFIG_PATH, data.pending_config.clone())?;
        }
        // TODO revert if config save fails
        // TODO Remove Pending Config File
        data.current_config = data.pending_config.clone();
        data.changelog = Vec::new();
        Ok(())
    }

    pub fn generate_current(&mut self) -> Result<(), ConfigError> {
        let data = self.shared_data.lock().unwrap();

        let mut errors = vec![];
        // TODO use null config as start? force full generation
        for apply_function in GENERATE_FUNCTIONS {
            match (apply_function)(
                false,
                data.current_config.clone(),
                data.current_config.clone(),
            ) {
                Ok(_) => info!("Generated"),
                Err(e) => {
                    error!(
                        "Ran into an error while generating, Proceeding with next System anyway. Error: {}",
                        e
                    );
                    errors.push(e);
                }
            }
        }

        if errors.len() != 0 {
            info!("Generate Finished with errors.");
            // TODO improve the errors
            Err(ConfigError::ApplyError(
                crate::apply::ApplyError::ConfigCheckFailed,
            ))
        } else {
            info!("Generate Done.");
            Ok(())
        }
    }

    pub fn discard_pending_changes(&mut self) -> Result<(), ConfigError> {
        let mut data = self.shared_data.lock().unwrap();
        // TODO Remove Pending Config File

        data.pending_config = data.current_config.clone();
        data.changelog = Vec::new();
        Ok(())
    }

    pub fn start_transaction(&mut self) -> ConfigTransaction<'_> {
        let data = self.shared_data.lock().unwrap();

        ConfigTransaction {
            config: data.pending_config.clone(),
            shared_data: data,
        }
    }
}

impl<'a> ConfigTransaction<'a> {
    pub fn data_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    pub fn commit(mut self, changelog: &mut Vec<Change>) -> Result<(), ConfigError> {
        validate_config(&self.config)?;

        let changes = self
            .config
            .diff(&self.shared_data.pending_config, &mut vec![]);

        if !changes.is_empty() {
            changelog.extend(changes);
            self.shared_data.pending_config = self.config.clone();
            if !self.shared_data.in_memory {
                write_config_to_file(PENDING_CONFIG_PATH, self.config.clone())?;
            }
        }

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
    validate_config(&conf)?;
    Ok(conf)
}

fn write_config_to_file(path: &str, conf: Config) -> Result<(), ConfigError> {
    let data: String = serde_json::to_string_pretty(&conf)?;
    fs::write(path, data)?;
    Ok(())
}

fn validate_config(conf: &Config) -> Result<(), ConfigError> {
    // Garde validation (with Config context)
    conf.validate()?;

    // Duplicate key and reference validation
    let errors = structdb_core::validate_all(conf);
    if !errors.is_empty() {
        return Err(ConfigError::ReferenceValidationError(errors));
    }
    Ok(())
}

pub fn generate_default_config(path: &str) -> Result<(), ConfigError> {
    let mut conf = Config::default();
    let hash = sha512_crypt::hash("nfsense")?;
    conf.config_version = 1;
    conf.system.users.push(crate::definitions::system::User {
        name: "root".to_string(),
        comment: "Default Admin".to_string(),
        hash: hash,
    });
    // Rule to allow Webinterface and SSH access after installation
    // TODO Restrict to Private Networks and nessesary ports once default Adresses and Services exist
    conf.firewall
        .inbound_rules
        .push(crate::definitions::firewall::InboundRule {
            name: "Default Allow Inbound".to_string(),
            services: vec![],
            source_addresses: vec![],
            negate_source: false,
            destination_addresses: vec![],
            negate_destination: false,
            comment: "Default Allow Inbound".to_string(),
            counter: true,
            log: true,
            verdict: crate::definitions::firewall::Verdict::Accept,
        });
    validate_config(&conf)?;
    write_config_to_file(path, conf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::system::User;
    use structdb_core::change::ChangeAction;

    fn create_test_config() -> Config {
        let mut conf = Config::default();
        conf.config_version = 1;
        conf.system.users.push(User {
            name: "root".to_string(),
            comment: "Default Admin".to_string(),
            hash: "somehash".to_string(),
        });
        conf
    }

    impl ConfigManager {
        fn new_for_test(current_config: Config, pending_config: Config) -> Self {
            Self {
                shared_data: Arc::new(Mutex::new(SharedData {
                    current_config,
                    pending_config,
                    changelog: Vec::new(),
                    in_memory: true,
                })),
            }
        }
    }

    #[test]
    fn test_transaction_commit_creates_changes() {
        let initial_config = create_test_config();
        let mut manager = ConfigManager::new_for_test(initial_config.clone(), initial_config);

        // Start a transaction
        let mut tx = manager.start_transaction();

        // Modify the config
        let new_user = User {
            name: "testuser".to_string(),
            comment: "Test user".to_string(),
            hash: "newhash".to_string(),
        };
        tx.data_mut().system.users.push(new_user);

        // Commit the transaction
        let mut data_changes = Vec::new();
        let commit_result = tx.commit(&mut data_changes);
        assert!(commit_result.is_ok());

        // The commit itself should produce changes, but not a ChangeSet
        assert_eq!(data_changes.len(), 1);

        // The API layer is responsible for wrapping this in a ChangeSet. Let's simulate that.
        manager.add_to_changelog(ChangeSet {
            user: "test@test.com".to_string(),
            timestamp: OffsetDateTime::now_utc(),
            changes: data_changes,
        });

        // Verify the changelog
        let changelog = manager.get_pending_changelog();
        assert_eq!(changelog.len(), 1);

        let change_set = &changelog[0];
        assert_eq!(change_set.user, "test@test.com");
        assert_eq!(change_set.changes.len(), 1);

        let change = &change_set.changes[0];
        assert_eq!(change.action, ChangeAction::Create);
        assert_eq!(change.path, "system.users");
        assert_eq!(change.id, "testuser");

        // Verify that the pending config inside the manager was updated
        let pending_config = manager.get_pending_config();
        assert_eq!(pending_config.system.users.len(), 2);
    }

    #[test]
    fn test_no_change_no_changes() {
        let initial_config = create_test_config();
        let mut manager = ConfigManager::new_for_test(initial_config.clone(), initial_config);

        // Start a transaction, but don't modify anything
        let tx = manager.start_transaction();

        let mut data_changes = Vec::new();
        let commit_result = tx.commit(&mut data_changes);
        assert!(commit_result.is_ok());

        // No changes should be detected
        assert!(data_changes.is_empty());

        // Simulate API layer logic
        if !data_changes.is_empty() {
            manager.add_to_changelog(ChangeSet {
                user: "test@test.com".to_string(),
                timestamp: OffsetDateTime::now_utc(),
                changes: data_changes,
            });
        }

        // Verify the changelog is empty
        let changelog = manager.get_pending_changelog();
        assert!(changelog.is_empty());
    }
}
