use octant_gui_core::{
    {CredentialCreationOptionsMethod, CredentialCreationOptionsTag}
    ,
    Method,
    PublicKeyCredentialCreationOptions,
};
use octant_object::define_class;

use crate::{handle, object, runtime::HasTypedHandle};

define_class! {
    #[derive(Debug)]
    pub class extends object {

    }
}

impl HasTypedHandle for Value {
    type TypeTag = CredentialCreationOptionsTag;
}

impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: object::Value::new(handle),
        }
    }
    pub fn public_key(&self, options: PublicKeyCredentialCreationOptions) {
        self.invoke(CredentialCreationOptionsMethod::PublicKey(options));
    }
}

impl Value {
    fn invoke(&self, method: CredentialCreationOptionsMethod) -> handle::Value {
        (**self).invoke(Method::CredentialCreationOptions(
            self.typed_handle(),
            method,
        ))
    }
}
