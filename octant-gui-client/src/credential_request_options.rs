use octant_gui_core::{
    CredentialRequestOptionsMethod,
    CredentialRequestOptionsTag, HandleId,
    PublicKeyCredentialRequestOptions,
};
use octant_object::define_class;

use crate::{export::Export, HasLocalType};
use crate::object::{Object, ObjectValue};
use crate::peer::ArcPeer;

define_class! {
    pub class CredentialRequestOptions extends Object {
        credential_request_options: web_sys::CredentialRequestOptions,
    }
}

impl CredentialRequestOptionsValue {
    pub fn new(handle: HandleId, credential_request_options: web_sys::CredentialRequestOptions) -> Self {
        CredentialRequestOptionsValue {
            parent: ObjectValue::new(handle, credential_request_options.clone().into()),
            credential_request_options,
        }
    }
    pub fn invoke_with(
        &self,
        method: &CredentialRequestOptionsMethod,
        _handle: HandleId,
    ) -> Option<ArcPeer> {
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
    pub fn native(&self) -> &web_sys::CredentialRequestOptions {
        &self.credential_request_options
    }
}

impl HasLocalType for CredentialRequestOptionsTag {
    type Local = dyn CredentialRequestOptions;
}
