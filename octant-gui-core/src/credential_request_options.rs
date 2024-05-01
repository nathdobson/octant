use serde::{Deserialize, Serialize};

use crate::{PublicKeyCredentialRequestOptions, TypeTag};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct CredentialRequestOptionsTag;

impl TypeTag for CredentialRequestOptionsTag {}

#[derive(Serialize, Deserialize, Debug)]
pub enum CredentialRequestOptionsMethod {
    PublicKey(PublicKeyCredentialRequestOptions),
}
