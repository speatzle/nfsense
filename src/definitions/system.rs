use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::get_thing;

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

get_thing!(User, get_user);
