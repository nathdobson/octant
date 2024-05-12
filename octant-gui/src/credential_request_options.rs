use octant_gui_core::{
    CredentialRequestOptionsMethod,
    CredentialRequestOptionsTag, Method,
    PublicKeyCredentialRequestOptions,
};
use octant_object::define_class;

use crate::handle::HandleValue;
use crate::object::{Object, ObjectValue};
use crate::runtime::HasTypedHandle;

define_class! {
    #[derive(Debug)]
    pub class CredentialRequestOptions extends Object {

    }
}

impl HasTypedHandle for CredentialRequestOptionsValue {
    type TypeTag = CredentialRequestOptionsTag;
}

impl CredentialRequestOptionsValue {
    pub fn new(handle: HandleValue) -> Self {
        CredentialRequestOptionsValue {
            parent: ObjectValue::new(handle),
        }
    }
    pub fn public_key(&self, options: PublicKeyCredentialRequestOptions) {
        self.invoke(CredentialRequestOptionsMethod::PublicKey(options));
    }
}

impl CredentialRequestOptionsValue {
    fn invoke(&self, method: CredentialRequestOptionsMethod) -> HandleValue {
        (**self).invoke(Method::CredentialRequestOptions(
            self.typed_handle(),
            method,
        ))
    }
}
