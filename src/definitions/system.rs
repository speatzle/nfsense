use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct System {
    pub users: HashMap<String, User>,
}

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct User {
    pub comment: String,
    pub hash: String,
}
