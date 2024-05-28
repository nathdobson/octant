use crate::{
    authentication_extensions_client_outputs::AuthenticationExtensionsClientOutputs,
    authenticator_assertion_response::AuthenticatorAssertionResponse,
    authenticator_attestation_response::AuthenticatorAttestationResponse,
    authenticator_response::AuthenticatorResponse, credential_data::CredentialData,
    public_key_credential::PublicKeyCredential,
};
use base64urlsafedata::Base64UrlSafeData;
use js_sys::{ArrayBuffer, Object, Uint8Array};
use octant_error::OctantError;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::console;

pub trait Import<T> {
    fn import(&self) -> T;
}

impl Import<CredentialData> for web_sys::Credential {
    fn import(&self) -> CredentialData {
        if let Some(this) = self.dyn_ref::<web_sys::PublicKeyCredential>() {
            CredentialData::PublicKeyCredential(this.import())
        } else {
            todo!();
        }
    }
}

impl Import<CredentialData> for JsValue {
    fn import(&self) -> CredentialData {
        self.clone()
            .dyn_into::<web_sys::Credential>()
            .unwrap()
            .import()
    }
}

impl Import<PublicKeyCredential> for web_sys::PublicKeyCredential {
    fn import(&self) -> PublicKeyCredential {
        PublicKeyCredential {
            id: self.id(),
            raw_id: self.raw_id().import(),
            response: self.response().import(),
            extensions: AuthenticationExtensionsClientOutputs {},
        }
    }
}

impl Import<Base64UrlSafeData> for ArrayBuffer {
    fn import(&self) -> Base64UrlSafeData {
        Base64UrlSafeData::from(Uint8Array::new(&self).to_vec())
    }
}

impl Import<AuthenticatorResponse> for web_sys::AuthenticatorResponse {
    fn import(&self) -> AuthenticatorResponse {
        if let Some(this) = self.dyn_ref::<web_sys::AuthenticatorAttestationResponse>() {
            AuthenticatorResponse::AuthenticatorAttestationResponse(this.import())
        } else if let Some(this) = self.dyn_ref::<web_sys::AuthenticatorAssertionResponse>() {
            AuthenticatorResponse::AuthenticatorAssertionResponse(this.import())
        } else {
            console::info_1(self);
            todo!();
        }
    }
}

impl Import<AuthenticatorAttestationResponse> for web_sys::AuthenticatorAttestationResponse {
    fn import(&self) -> AuthenticatorAttestationResponse {
        AuthenticatorAttestationResponse {
            attestation_object: self.attestation_object().import(),
            client_data_json: self.client_data_json().import(),
        }
    }
}

impl Import<AuthenticatorAssertionResponse> for web_sys::AuthenticatorAssertionResponse {
    fn import(&self) -> AuthenticatorAssertionResponse {
        AuthenticatorAssertionResponse {
            authenticator_data: self.authenticator_data().import(),
            client_data_json: self.client_data_json().import(),
            signature: self.signature().import(),
            user_handle: self.user_handle().import(),
        }
    }
}

impl<T1, T2> Import<Option<T2>> for Option<T1>
where
    T1: Import<T2>,
{
    fn import(&self) -> Option<T2> {
        self.as_ref().map(|x| x.import())
    }
}

impl<T1, T2, E1, E2> Import<Result<T2, E2>> for Result<T1, E1>
where
    T1: Import<T2>,
    E1: Import<E2>,
{
    fn import(&self) -> Result<T2, E2> {
        match self {
            Ok(value) => Ok(value.import()),
            Err(error) => Err(error.import()),
        }
    }
}

impl Import<OctantError> for JsValue {
    fn import(&self) -> OctantError {
        OctantError::from(self.clone())
    }
}
