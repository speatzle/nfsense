use super::config::Config;
use crate::validation;
use garde::Validate;
use serde::{Deserialize, Serialize};
use structdb_macros::StructDb;

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Default, Debug)]
#[garde(context(Config))]
pub struct System {
    #[collection]
    #[garde(dive)]
    pub users: Vec<User>,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Default, Debug)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct User {
    #[garde(custom(validation::validate_name))]
    pub name: String,
    pub comment: String,
    pub hash: String,
}
