use serde::{Deserialize, Serialize};

use crate::{credential::Credential, error::Error, TypeTag};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct CredentialPromiseTag;

impl TypeTag for CredentialPromiseTag {}

#[derive(Serialize, Deserialize, Debug)]
pub enum CredentialPromiseMethod {
    Wait,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CredentialPromiseUpMessage {
    Done(Result<Credential, Error>),
}
