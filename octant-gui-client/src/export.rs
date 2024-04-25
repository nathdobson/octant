use base64urlsafedata::Base64UrlSafeData;
use js_sys::{Array, Object, Reflect, Uint8Array};

use octant_gui_core::{
    PubKeyCredParams, PublicKeyCredentialCreationOptions, PublicKeyCredentialRpEntity,
    PublicKeyCredentialUserEntity,
};

pub trait Export<T> {
    fn export(&self) -> T;
}

impl Export<web_sys::PublicKeyCredentialCreationOptions> for PublicKeyCredentialCreationOptions {
    fn export(&self) -> web_sys::PublicKeyCredentialCreationOptions {
        web_sys::PublicKeyCredentialCreationOptions::new(
            &self.challenge.export(),
            &self.pub_key_cred_params.export(),
            // &serde_wasm_bindgen::to_value(&self.pub_key_cred_params).unwrap(),
            &self.rp.export(),
            &self.user.export(),
        )
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
            &Uint8Array::from(&*self.id.0),
        )
    }
}

impl Export<Uint8Array> for Base64UrlSafeData {
    fn export(&self) -> Uint8Array {
        Uint8Array::from(&*self.0)
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
