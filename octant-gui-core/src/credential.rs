use serde::{Deserialize, Serialize};

use crate::{CredentialData, TypeTag};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct CredentialTag;

impl TypeTag for CredentialTag {}

#[derive(Serialize, Deserialize, Debug)]
pub enum CredentialMethod {
    Materialize,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CredentialUpMessage {
    Materialize(CredentialData),
}
