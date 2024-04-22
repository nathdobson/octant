use base64urlsafedata::Base64UrlSafeData;
use js_sys::{ArrayBuffer, Uint8Array};
use octant_gui_core::{
    AuthenticationExtensionsClientOutputs, AuthenticatorAttestationResponse, AuthenticatorResponse,
    Credential,
};
use wasm_bindgen::JsCast;
use web_sys::PublicKeyCredential;

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

impl Import<octant_gui_core::PublicKeyCredential>
    for web_sys::PublicKeyCredential
{
    fn import(&self) -> octant_gui_core::PublicKeyCredential {
        octant_gui_core::PublicKeyCredential {
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

impl Import<octant_gui_core::AuthenticatorResponse>
    for web_sys::AuthenticatorResponse
{
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
