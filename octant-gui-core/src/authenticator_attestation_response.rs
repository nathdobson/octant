use base64urlsafedata::Base64UrlSafeData;
use serde::{Deserialize, Serialize};
use crate::authenticator_transport::AuthenticatorTransport;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticatorAttestationResponse {
    pub attestation_object: Base64UrlSafeData,
    pub client_data_json: Base64UrlSafeData,
}
