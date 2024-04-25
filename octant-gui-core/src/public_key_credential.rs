use base64urlsafedata::Base64UrlSafeData;
use serde::{Deserialize, Serialize};

use crate::authenticator_response::AuthenticatorResponse;
use crate::registration_extensions_client_outputs::RegistrationExtensionsClientOutputs;

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicKeyCredential {
    pub id: String,
    pub raw_id: Base64UrlSafeData,
    pub response: AuthenticatorResponse,
    pub extensions: RegistrationExtensionsClientOutputs,
}