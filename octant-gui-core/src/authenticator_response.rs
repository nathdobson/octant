use serde::{Deserialize, Serialize};
use crate::authenticator_attestation_response::AuthenticatorAttestationResponse;

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthenticatorResponse {
    AuthenticatorAttestationResponse(AuthenticatorAttestationResponse),
}
