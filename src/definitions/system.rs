use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct System {
    pub users: Vec<User>,
}

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct User {
    pub name: String,
    pub comment: String,
    pub hash: String,
}
