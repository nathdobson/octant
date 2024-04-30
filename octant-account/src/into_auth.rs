use webauthn_rs::prelude::RegisterPublicKeyCredential;
use webauthn_rs_core::proto::AuthenticatorAttestationResponseRaw;

use octant_gui_core::{AuthenticatorAttestationResponse, AuthenticatorResponse, Credential, PublicKeyCredential, RegistrationExtensionsClientOutputs};

pub trait IntoAuth<O> {
    fn into_auth(self) -> O;
}

impl IntoAuth<RegisterPublicKeyCredential> for Credential {
    fn into_auth(self) -> RegisterPublicKeyCredential {
        match self {
            Credential::PublicKeyCredential(credential) => credential.into_auth(),
        }
    }
}

impl IntoAuth<RegisterPublicKeyCredential> for PublicKeyCredential {
    fn into_auth(self) -> RegisterPublicKeyCredential {
        RegisterPublicKeyCredential {
            id: self.id,
            raw_id: self.raw_id,
            response: self.response.into_auth(),
            type_: "PublicKeyCredential".to_string(),
            extensions: self.extensions.into_auth(),
        }
    }
}

impl IntoAuth<webauthn_rs_core::proto::AuthenticatorAttestationResponseRaw>
for AuthenticatorResponse
{
    fn into_auth(self) -> webauthn_rs_core::proto::AuthenticatorAttestationResponseRaw {
        match self {
            AuthenticatorResponse::AuthenticatorAttestationResponse(resp) => resp.into_auth(),
        }
    }
}

impl IntoAuth<webauthn_rs_core::proto::RegistrationExtensionsClientOutputs>
for RegistrationExtensionsClientOutputs
{
    fn into_auth(self) -> webauthn_rs_core::proto::RegistrationExtensionsClientOutputs {
        webauthn_rs_core::proto::RegistrationExtensionsClientOutputs {
            appid: None,
            cred_props: None,
            hmac_secret: None,
            cred_protect: None,
            min_pin_length: None,
        }
    }
}

impl IntoAuth<webauthn_rs_core::proto::AuthenticatorAttestationResponseRaw>
for AuthenticatorAttestationResponse
{
    fn into_auth(self) -> AuthenticatorAttestationResponseRaw {
        AuthenticatorAttestationResponseRaw {
            attestation_object: self.attestation_object,
            client_data_json: self.client_data_json,
            transports: None,
        }
    }
}