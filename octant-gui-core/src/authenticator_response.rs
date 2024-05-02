use serde::{Deserialize, Serialize};

use crate::authenticator_attestation_response::AuthenticatorAttestationResponse;
use crate::AuthenticatorAssertionResponse;

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthenticatorResponse {
    AuthenticatorAttestationResponse(AuthenticatorAttestationResponse),
    AuthenticatorAssertionResponse(AuthenticatorAssertionResponse),
}
