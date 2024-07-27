use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct System {
    #[garde(dive)]
    pub users: Vec<User>,
}

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
#[garde(allow_unvalidated)]
pub struct User {
    pub name: String,
    pub comment: String,
    pub hash: String,
}
