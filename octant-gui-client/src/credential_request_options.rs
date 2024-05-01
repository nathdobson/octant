use std::sync::Arc;

use web_sys::CredentialRequestOptions;

use octant_gui_core::{
    CredentialRequestOptionsMethod,
    CredentialRequestOptionsTag, HandleId,
    PublicKeyCredentialRequestOptions,
};
use octant_object::define_class;

use crate::{export::Export, HasLocalType, object, peer};

define_class! {
    pub class extends object {
        credential_request_options: CredentialRequestOptions,
    }
}

impl Value {
    pub fn new(handle: HandleId, credential_request_options: CredentialRequestOptions) -> Self {
        Value {
            parent: object::Value::new(handle, credential_request_options.clone().into()),
            credential_request_options,
        }
    }
    pub fn invoke_with(
        &self,
        method: &CredentialRequestOptionsMethod,
        _handle: HandleId,
    ) -> Option<Arc<dyn peer::Trait>> {
        match method {
            CredentialRequestOptionsMethod::PublicKey(options) => {
                self.public_key(options);
                None
            }
        }
    }
    pub fn public_key(&self, options: &PublicKeyCredentialRequestOptions) {
        self.credential_request_options
            .clone()
            .public_key(&options.export());
    }
    pub fn native(&self) -> &CredentialRequestOptions {
        &self.credential_request_options
    }
}

impl HasLocalType for CredentialRequestOptionsTag {
    type Local = dyn Trait;
}
