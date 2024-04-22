use std::sync::Arc;

use web_sys::{
    CredentialCreationOptions,
    js_sys::Uint8Array,
};

use octant_gui_core::{
    credential_creation_options::{CredentialCreationOptionsMethod, CredentialCreationOptionsTag}
    ,
    HandleId
    ,
    public_key_credential_creation_options::PublicKeyCredentialCreationOptions,
};
use octant_object::define_class;

use crate::{HasLocalType, object, peer};

define_class! {
    pub class extends object {
        credentials_creation_options: CredentialCreationOptions,
    }
}

impl Value {
    pub fn new(handle: HandleId, credentials_creation_options: CredentialCreationOptions) -> Self {
        Value {
            parent: object::Value::new(handle, credentials_creation_options.clone().into()),
            credentials_creation_options,
        }
    }
    pub fn invoke_with(
        &self,
        method: &CredentialCreationOptionsMethod,
        handle: HandleId,
    ) -> Option<Arc<dyn peer::Trait>> {
        match method {
            CredentialCreationOptionsMethod::PublicKey(options) => {
                self.public_key(options);
                None
            }
        }
    }
    pub fn public_key(&self, options: &PublicKeyCredentialCreationOptions) {
        let mut rp = web_sys::PublicKeyCredentialRpEntity::new(&options.rp.name);
        if let Some(id) = &options.rp.id {
            rp.id(id);
        }
        if let Some(icon) = &options.rp.icon {
            rp.icon(icon);
        }
        let user = web_sys::PublicKeyCredentialUserEntity::new(
            &options.user.name,
            &options.user.display_name,
            &Uint8Array::from(&*options.user.id.0),
        );
        let cco = web_sys::PublicKeyCredentialCreationOptions::new(
            &Uint8Array::from(&*options.challenge.0),
            &serde_wasm_bindgen::to_value(&options.pub_key_cred_params).unwrap(),
            &rp,
            &user,
        );
        web_sys::console::log_1(&cco);
        self.credentials_creation_options.clone().public_key(&cco);
    }
    pub fn native(&self) -> &CredentialCreationOptions {
        &self.credentials_creation_options
    }
}

impl HasLocalType for CredentialCreationOptionsTag {
    type Local = dyn Trait;
}
