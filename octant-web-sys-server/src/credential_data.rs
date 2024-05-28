use octant_serde::{DeserializeContext, DeserializeWith};
use serde::{Deserialize, Deserializer, Serialize};

use crate::public_key_credential::PublicKeyCredential;

#[derive(Serialize, Deserialize, Debug)]
pub enum CredentialData {
    PublicKeyCredential(PublicKeyCredential),
}

impl<'de> DeserializeWith<'de> for CredentialData {
    fn deserialize_with<D: Deserializer<'de>>(
        ctx: &DeserializeContext,
        d: D,
    ) -> Result<Self, D::Error> {
        Self::deserialize(d)
    }
}
