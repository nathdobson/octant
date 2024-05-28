use base64urlsafedata::Base64UrlSafeData;
use octant_serde::{DeserializeContext, DeserializeWith};
use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    allow_credentials::AllowCredentials,
    authentication_extensions_client_inputs::AuthenticationExtensionsClientInputs,
    user_verification_requirement::UserVerificationRequirement,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicKeyCredentialRequestOptions {
    pub challenge: Base64UrlSafeData,
    pub timeout: Option<u32>,
    pub rp_id: String,
    pub allow_credentials: Vec<AllowCredentials>,
    pub user_verification: UserVerificationRequirement,
    pub extensions: Option<AuthenticationExtensionsClientInputs>,
}

impl<'de> DeserializeWith<'de> for PublicKeyCredentialRequestOptions {
    fn deserialize_with<D: Deserializer<'de>>(
        ctx: &DeserializeContext,
        d: D,
    ) -> Result<Self, D::Error> {
        Self::deserialize(d)
    }
}
