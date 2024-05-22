use serde::{Deserialize, Serialize};

use crate::{
    authenticator_assertion_response::AuthenticatorAssertionResponse,
    authenticator_attestation_response::AuthenticatorAttestationResponse,
};

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthenticatorResponse {
    AuthenticatorAttestationResponse(AuthenticatorAttestationResponse),
    AuthenticatorAssertionResponse(AuthenticatorAssertionResponse),
}
