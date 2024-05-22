use webauthn_rs_proto::{AuthenticatorAssertionResponseRaw, AuthenticatorAttestationResponseRaw};

use octant_web_sys_server::{
    authentication_extensions_client_outputs::AuthenticationExtensionsClientOutputs,
    authenticator_assertion_response::AuthenticatorAssertionResponse,
    authenticator_attestation_response::AuthenticatorAttestationResponse,
    authenticator_response::AuthenticatorResponse, credential_data::CredentialData,
    public_key_credential::PublicKeyCredential,
};

pub trait IntoAuth<O> {
    fn into_auth(self) -> O;
}

impl IntoAuth<webauthn_rs_proto::RegisterPublicKeyCredential> for CredentialData {
    fn into_auth(self) -> webauthn_rs_proto::RegisterPublicKeyCredential {
        match self {
            CredentialData::PublicKeyCredential(credential) => credential.into_auth(),
        }
    }
}

impl IntoAuth<webauthn_rs_proto::PublicKeyCredential> for CredentialData {
    fn into_auth(self) -> webauthn_rs_proto::PublicKeyCredential {
        match self {
            CredentialData::PublicKeyCredential(credential) => credential.into_auth(),
        }
    }
}

impl IntoAuth<webauthn_rs_proto::RegisterPublicKeyCredential> for PublicKeyCredential {
    fn into_auth(self) -> webauthn_rs_proto::RegisterPublicKeyCredential {
        webauthn_rs_proto::RegisterPublicKeyCredential {
            id: self.id,
            raw_id: self.raw_id,
            response: self.response.into_auth(),
            type_: "PublicKeyCredential".to_string(),
            extensions: self.extensions.into_auth(),
        }
    }
}

impl IntoAuth<webauthn_rs_proto::PublicKeyCredential> for PublicKeyCredential {
    fn into_auth(self) -> webauthn_rs_proto::PublicKeyCredential {
        webauthn_rs_proto::PublicKeyCredential {
            id: self.id,
            raw_id: self.raw_id,
            response: self.response.into_auth(),
            extensions: self.extensions.into_auth(),
            type_: "PublicKeyCredential".to_string(),
        }
    }
}

impl IntoAuth<webauthn_rs_proto::AuthenticatorAttestationResponseRaw> for AuthenticatorResponse {
    fn into_auth(self) -> webauthn_rs_proto::AuthenticatorAttestationResponseRaw {
        match self {
            AuthenticatorResponse::AuthenticatorAttestationResponse(resp) => resp.into_auth(),
            AuthenticatorResponse::AuthenticatorAssertionResponse(resp) => panic!("bad response"),
        }
    }
}

impl IntoAuth<webauthn_rs_proto::AuthenticatorAssertionResponseRaw> for AuthenticatorResponse {
    fn into_auth(self) -> AuthenticatorAssertionResponseRaw {
        match self {
            AuthenticatorResponse::AuthenticatorAttestationResponse(resp) => panic!("bad response"),
            AuthenticatorResponse::AuthenticatorAssertionResponse(resp) => resp.into_auth(),
        }
    }
}

impl IntoAuth<webauthn_rs_proto::RegistrationExtensionsClientOutputs>
    for AuthenticationExtensionsClientOutputs
{
    fn into_auth(self) -> webauthn_rs_proto::RegistrationExtensionsClientOutputs {
        webauthn_rs_proto::RegistrationExtensionsClientOutputs {
            appid: None,
            cred_props: None,
            hmac_secret: None,
            cred_protect: None,
            min_pin_length: None,
        }
    }
}

impl IntoAuth<webauthn_rs_proto::AuthenticationExtensionsClientOutputs>
    for AuthenticationExtensionsClientOutputs
{
    fn into_auth(self) -> webauthn_rs_proto::AuthenticationExtensionsClientOutputs {
        webauthn_rs_proto::AuthenticationExtensionsClientOutputs {
            appid: None,
            hmac_get_secret: None,
        }
    }
}

impl IntoAuth<webauthn_rs_proto::AuthenticatorAttestationResponseRaw>
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

impl IntoAuth<webauthn_rs_proto::AuthenticatorAssertionResponseRaw>
    for AuthenticatorAssertionResponse
{
    fn into_auth(self) -> webauthn_rs_proto::AuthenticatorAssertionResponseRaw {
        webauthn_rs_proto::AuthenticatorAssertionResponseRaw {
            authenticator_data: self.authenticator_data,
            client_data_json: self.client_data_json,
            signature: self.signature,
            user_handle: self.user_handle,
        }
    }
}
