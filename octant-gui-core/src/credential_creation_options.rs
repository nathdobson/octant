use serde::{Deserialize, Serialize};
use crate::public_key_credential_creation_options::PublicKeyCredentialCreationOptions;

use crate::TypeTag;

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct CredentialCreationOptionsTag;

impl TypeTag for CredentialCreationOptionsTag {}

#[derive(Serialize, Deserialize, Debug)]
pub enum CredentialCreationOptionsMethod {
    PublicKey(PublicKeyCredentialCreationOptions),
}
