use octant_gui_core::{
    CredentialRequestOptionsMethod,
    CredentialRequestOptionsTag, Method,
    PublicKeyCredentialRequestOptions,
};
use octant_object::define_class;

use crate::{handle, object, runtime::HasTypedHandle};

define_class! {
    #[derive(Debug)]
    pub class extends object {

    }
}

impl HasTypedHandle for Value {
    type TypeTag = CredentialRequestOptionsTag;
}

impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: object::Value::new(handle),
        }
    }
    pub fn public_key(&self, options: PublicKeyCredentialRequestOptions) {
        self.invoke(CredentialRequestOptionsMethod::PublicKey(options));
    }
}

impl Value {
    fn invoke(&self, method: CredentialRequestOptionsMethod) -> handle::Value {
        (**self).invoke(Method::CredentialRequestOptions(
            self.typed_handle(),
            method,
        ))
    }
}
