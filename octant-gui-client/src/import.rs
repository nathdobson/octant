use base64urlsafedata::Base64UrlSafeData;
use js_sys::{ArrayBuffer, Object, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::PublicKeyCredential;

use octant_gui_core::{AuthenticatorAttestationResponse, AuthenticatorResponse, Error, RegistrationExtensionsClientOutputs};

pub trait Import<T> {
    fn import(&self) -> T;
}

impl Import<octant_gui_core::Credential> for web_sys::Credential {
    fn import(&self) -> octant_gui_core::Credential {
        if let Some(this) = self.dyn_ref::<PublicKeyCredential>() {
            octant_gui_core::Credential::PublicKeyCredential(this.import())
        } else {
            todo!();
        }
    }
}

impl Import<octant_gui_core::PublicKeyCredential> for web_sys::PublicKeyCredential {
    fn import(&self) -> octant_gui_core::PublicKeyCredential {
        octant_gui_core::PublicKeyCredential {
            id: self.id(),
            raw_id: self.raw_id().import(),
            response: self.response().import(),
            extensions: RegistrationExtensionsClientOutputs {},
        }
    }
}

impl Import<Base64UrlSafeData> for ArrayBuffer {
    fn import(&self) -> Base64UrlSafeData {
        Base64UrlSafeData::from(Uint8Array::new(&self).to_vec())
    }
}

impl Import<octant_gui_core::AuthenticatorResponse> for web_sys::AuthenticatorResponse {
    fn import(&self) -> AuthenticatorResponse {
        if let Some(this) = self.dyn_ref::<web_sys::AuthenticatorAttestationResponse>() {
            octant_gui_core::AuthenticatorResponse::AuthenticatorAttestationResponse(this.import())
        } else {
            todo!();
        }
    }
}

impl Import<octant_gui_core::AuthenticatorAttestationResponse>
    for web_sys::AuthenticatorAttestationResponse
{
    fn import(&self) -> AuthenticatorAttestationResponse {
        AuthenticatorAttestationResponse {
            attestation_object: self.attestation_object().import(),
            client_data_json: self.client_data_json().import(),
        }
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

impl Import<Error> for JsValue {
    fn import(&self) -> Error {
        Error {
            content: self.clone().dyn_ref::<Object>().unwrap().to_string().into(),
        }
    }
}
