use std::sync::Arc;

use octant_gui_core::{
    CredentialCreationOptionsMethod, CredentialCreationOptionsTag, HandleId,
    PublicKeyCredentialCreationOptions,
};
use octant_object::define_class;

use crate::{
    export::Export,
    object,
    object::{Object, ObjectValue},
    peer, HasLocalType,
};
use crate::peer::ArcPeer;

define_class! {
    pub class CredentialCreationOptions extends Object {
        credential_creation_options: web_sys::CredentialCreationOptions,
    }
}

impl CredentialCreationOptionsValue {
    pub fn new(
        handle: HandleId,
        credential_creation_options: web_sys::CredentialCreationOptions,
    ) -> Self {
        CredentialCreationOptionsValue {
            parent: ObjectValue::new(handle, credential_creation_options.clone().into()),
            credential_creation_options,
        }
    }
    pub fn invoke_with(
        &self,
        method: &CredentialCreationOptionsMethod,
        _handle: HandleId,
    ) -> Option<ArcPeer> {
        match method {
            CredentialCreationOptionsMethod::PublicKey(options) => {
                self.public_key(options);
                None
            }
        }
    }
    pub fn public_key(&self, options: &PublicKeyCredentialCreationOptions) {
        self.credential_creation_options
            .clone()
            .public_key(&options.export());
    }
    pub fn native(&self) -> &web_sys::CredentialCreationOptions {
        &self.credential_creation_options
    }
}

impl HasLocalType for CredentialCreationOptionsTag {
    type Local = dyn CredentialCreationOptions;
}
