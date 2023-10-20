pub mod network;
pub mod object;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Default, Debug)]
pub struct Config {
    pub config_version: u64,
    pub network: network::Network,
    pub object: object::Object,
}
