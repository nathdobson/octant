use octant_gui_core::{
    {CredentialCreationOptionsMethod, CredentialCreationOptionsTag}
    ,
    Method,
    PublicKeyCredentialCreationOptions,
};
use octant_object::define_class;

use crate::{handle, object, runtime::HasTypedHandle};
use crate::handle::HandleValue;
use crate::object::{Object, ObjectValue};

define_class! {
    #[derive(Debug)]
    pub class CredentialCreationOptions extends Object {

    }
}

impl HasTypedHandle for CredentialCreationOptionsValue {
    type TypeTag = CredentialCreationOptionsTag;
}

impl CredentialCreationOptionsValue {
    pub fn new(handle: HandleValue) -> Self {
        CredentialCreationOptionsValue {
            parent: ObjectValue::new(handle),
        }
    }
    pub fn public_key(&self, options: PublicKeyCredentialCreationOptions) {
        self.invoke(CredentialCreationOptionsMethod::PublicKey(options));
    }
}

impl CredentialCreationOptionsValue {
    fn invoke(&self, method: CredentialCreationOptionsMethod) -> HandleValue {
        (**self).invoke(Method::CredentialCreationOptions(
            self.typed_handle(),
            method,
        ))
    }
}
