use std::sync::Arc;

use web_sys::CredentialCreationOptions;

use octant_gui_core::{
    {CredentialCreationOptionsMethod, CredentialCreationOptionsTag}
    ,
    HandleId
    ,
    PublicKeyCredentialCreationOptions,
};
use octant_object::define_class;

use crate::{HasLocalType, object, peer};
use crate::export::Export;

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
        _handle: HandleId,
    ) -> Option<Arc<dyn peer::Trait>> {
        match method {
            CredentialCreationOptionsMethod::PublicKey(options) => {
                self.public_key(options);
                None
            }
        }
    }
    pub fn public_key(&self, options: &PublicKeyCredentialCreationOptions) {
        self.credentials_creation_options.clone().public_key(&options.export());
    }
    pub fn native(&self) -> &CredentialCreationOptions {
        &self.credentials_creation_options
    }
}

impl HasLocalType for CredentialCreationOptionsTag {
    type Local = dyn Trait;
}
