use base64urlsafedata::Base64UrlSafeData;

use js_sys::{Array, Object, Reflect, Uint8Array};
use wasm_bindgen::JsValue;

use crate::{
    allow_credentials::AllowCredentials, allow_credentials_type::AllowCredentialsType,
    attestation_conveyance_preference::AttestationConveyancePreference,
    authentication_extensions_client_inputs::AuthenticationExtensionsClientInputs,
    authenticator_attachment::AuthenticatorAttachment,
    authenticator_selection_criteria::AuthenticatorSelectionCriteria,
    authenticator_transport::AuthenticatorTransport, pub_key_cred_params::PubKeyCredParams,
    public_key_credential_creation_options::PublicKeyCredentialCreationOptions,
    public_key_credential_request_options::PublicKeyCredentialRequestOptions,
    public_key_credential_rp_entity::PublicKeyCredentialRpEntity,
    public_key_credential_user_entity::PublicKeyCredentialUserEntity,
    user_verification_requirement::UserVerificationRequirement,
};

pub trait Export<T> {
    fn export(&self) -> T;
}

impl Export<web_sys::PublicKeyCredentialCreationOptions> for PublicKeyCredentialCreationOptions {
    fn export(&self) -> web_sys::PublicKeyCredentialCreationOptions {
        let mut result = web_sys::PublicKeyCredentialCreationOptions::new(
            &self.challenge.export(),
            &self.pub_key_cred_params.export(),
            // &serde_wasm_bindgen::to_value(&self.pub_key_cred_params).unwrap(),
            &self.rp.export(),
            &self.user.export(),
        );
        if let Some(authenticator_selection) = &self.authenticator_selection {
            result.authenticator_selection(&authenticator_selection.export());
        }
        result.attestation(self.attestation.export());
        if let Some(extensions) = &self.extensions {
            result.extensions(&extensions.export());
        }
        if let Some(timeout) = self.timeout {
            result.timeout(timeout);
        }
        result
    }
}

impl Export<web_sys::PublicKeyCredentialRequestOptions> for PublicKeyCredentialRequestOptions {
    fn export(&self) -> web_sys::PublicKeyCredentialRequestOptions {
        let mut result = web_sys::PublicKeyCredentialRequestOptions::new(&self.challenge.export());
        result.allow_credentials(&self.allow_credentials.export());
        result.challenge(&self.challenge.export());
        if let Some(extensions) = &self.extensions {
            result.extensions(&extensions.export());
        }
        result.rp_id(&self.rp_id);
        if let Some(timeout) = self.timeout {
            result.timeout(timeout);
        }
        result.user_verification(self.user_verification.export());
        result
    }
}

impl Export<web_sys::AuthenticatorSelectionCriteria> for AuthenticatorSelectionCriteria {
    fn export(&self) -> web_sys::AuthenticatorSelectionCriteria {
        let mut result = web_sys::AuthenticatorSelectionCriteria::new();
        if let Some(authenticator_attachment) = &self.authenticator_attachment {
            result.authenticator_attachment(authenticator_attachment.export());
        }
        result.require_resident_key(self.require_resident_key);
        result.user_verification(self.user_verification.export());
        result
    }
}

impl Export<web_sys::UserVerificationRequirement> for UserVerificationRequirement {
    fn export(&self) -> web_sys::UserVerificationRequirement {
        match self {
            UserVerificationRequirement::Required => web_sys::UserVerificationRequirement::Required,
            UserVerificationRequirement::Preferred => {
                web_sys::UserVerificationRequirement::Preferred
            }
            UserVerificationRequirement::Discouraged => {
                web_sys::UserVerificationRequirement::Discouraged
            }
        }
    }
}

impl Export<web_sys::AuthenticatorAttachment> for AuthenticatorAttachment {
    fn export(&self) -> web_sys::AuthenticatorAttachment {
        match self {
            AuthenticatorAttachment::Platform => web_sys::AuthenticatorAttachment::Platform,
            AuthenticatorAttachment::CrossPlatform => {
                web_sys::AuthenticatorAttachment::CrossPlatform
            }
        }
    }
}

impl Export<web_sys::AttestationConveyancePreference> for AttestationConveyancePreference {
    fn export(&self) -> web_sys::AttestationConveyancePreference {
        match self {
            AttestationConveyancePreference::None => web_sys::AttestationConveyancePreference::None,
            AttestationConveyancePreference::Indirect => {
                web_sys::AttestationConveyancePreference::Indirect
            }
            AttestationConveyancePreference::Direct => {
                web_sys::AttestationConveyancePreference::Direct
            }
        }
    }
}

impl Export<web_sys::AuthenticationExtensionsClientInputs>
    for AuthenticationExtensionsClientInputs
{
    fn export(&self) -> web_sys::AuthenticationExtensionsClientInputs {
        web_sys::AuthenticationExtensionsClientInputs::new()
    }
}

impl Export<web_sys::PublicKeyCredentialRpEntity> for PublicKeyCredentialRpEntity {
    fn export(&self) -> web_sys::PublicKeyCredentialRpEntity {
        let mut rp = web_sys::PublicKeyCredentialRpEntity::new(&self.name);
        if let Some(id) = &self.id {
            rp.id(id);
        }
        if let Some(icon) = &self.icon {
            rp.icon(icon);
        }
        rp
    }
}

impl Export<web_sys::PublicKeyCredentialUserEntity> for PublicKeyCredentialUserEntity {
    fn export(&self) -> web_sys::PublicKeyCredentialUserEntity {
        web_sys::PublicKeyCredentialUserEntity::new(
            &self.name,
            &self.display_name,
            &Uint8Array::from(&**self.id),
        )
    }
}

impl Export<Uint8Array> for Base64UrlSafeData {
    fn export(&self) -> Uint8Array {
        Uint8Array::from(&***self)
    }
}

impl Export<Array> for Vec<PubKeyCredParams> {
    fn export(&self) -> Array {
        self.into_iter().map(|x| x.export()).collect()
    }
}

impl Export<Object> for PubKeyCredParams {
    fn export(&self) -> Object {
        let ret = Object::new();
        Reflect::set(&ret, &"alg".into(), &self.alg.into()).unwrap();
        Reflect::set(&ret, &"type".into(), &(&self.typ).into()).unwrap();
        ret
    }
}

impl Export<Object> for AllowCredentials {
    fn export(&self) -> Object {
        let ret = Object::new();
        Reflect::set(&ret, &"id".into(), &self.id.export()).unwrap();
        Reflect::set(&ret, &"transports".into(), &self.transports.export()).unwrap();
        Reflect::set(&ret, &"type".into(), &self.typ.export()).unwrap();
        ret
    }
}

impl Export<JsValue> for Vec<AuthenticatorTransport> {
    fn export(&self) -> JsValue {
        self.into_iter()
            .map(|x| x.export())
            .collect::<Array>()
            .into()
    }
}

impl Export<JsValue> for AuthenticatorTransport {
    fn export(&self) -> JsValue {
        match self {
            AuthenticatorTransport::Usb => "usb",
            AuthenticatorTransport::Nfc => "nfc",
            AuthenticatorTransport::Ble => "ble",
            AuthenticatorTransport::Internal => "internal",
            AuthenticatorTransport::Hybrid => "hybrid",
            AuthenticatorTransport::Test => "test",
            AuthenticatorTransport::Unknown => "unknown",
        }
        .into()
    }
}

impl Export<JsValue> for Vec<AllowCredentials> {
    fn export(&self) -> JsValue {
        self.into_iter()
            .map(|x| x.export())
            .collect::<Array>()
            .into()
    }
}

impl Export<JsValue> for AllowCredentialsType {
    fn export(&self) -> JsValue {
        match self {
            AllowCredentialsType::PublicKey => "public-key".into(),
        }
    }
}
