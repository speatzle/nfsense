use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Default, Debug)]
pub struct System {
    pub users: HashMap<String, User>,
}

#[derive(Serialize, Deserialize, Validate, Default, Debug)]
pub struct User {
    pub comment: String,
    pub hash: String,
    pub salt: String,
}
