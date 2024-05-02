use base64urlsafedata::Base64UrlSafeData;
use serde::{Deserialize, Serialize};

use crate::AuthenticationExtensionsClientOutputs;
use crate::authenticator_response::AuthenticatorResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicKeyCredential {
    pub id: String,
    pub raw_id: Base64UrlSafeData,
    pub response: AuthenticatorResponse,
    pub extensions: AuthenticationExtensionsClientOutputs,
}
