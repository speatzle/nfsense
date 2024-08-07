use super::config::Config;
use crate::validation;
use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
#[garde(context(Config))]
pub struct System {
    #[garde(dive)]
    pub users: Vec<User>,
}

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct User {
    #[garde(custom(validation::validate_name))]
    pub name: String,
    pub comment: String,
    pub hash: String,
}
