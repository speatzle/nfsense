use serde::{Deserialize, Serialize};
use validator::Validate;

// Referencing
use crate::definitions::config::Config;
use crate::definitions::Referenceable;
use crate::definitions::References;
use crate::{impl_referenceable_trait, impl_references_trait};

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct System {
    pub users: Vec<User>,
}

type Users = Vec<User>;
impl_referenceable_trait!(Users, User);

pub type UserReference = String;
impl_references_trait!(UserReference, User, system.users);

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct User {
    pub name: String,
    pub comment: String,
    pub hash: String,
}
