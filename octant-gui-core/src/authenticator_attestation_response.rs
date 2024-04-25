use base64urlsafedata::Base64UrlSafeData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticatorAttestationResponse {
    pub attestation_object: Base64UrlSafeData,
    pub client_data_json: Base64UrlSafeData,
}
