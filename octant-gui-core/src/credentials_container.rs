use serde::{Deserialize, Serialize};

use crate::{CredentialRequestOptionsTag, TypedHandle, TypeTag};
use crate::credential_creation_options::CredentialCreationOptionsTag;

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct CredentialsContainerTag;

impl TypeTag for CredentialsContainerTag {}

#[derive(Serialize, Deserialize, Debug)]
pub enum CredentialsContainerMethod {
    CreateWithOptions(
        TypedHandle<CredentialCreationOptionsTag>,
    ),
    GetWithOptions(
        TypedHandle<CredentialRequestOptionsTag>,
    ),
}
