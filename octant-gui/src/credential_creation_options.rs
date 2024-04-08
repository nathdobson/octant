use crate::{handle, object, runtime::HasTypedHandle};
use octant_gui_core::{
    credential_creation_options::{CredentialCreationOptionsMethod, CredentialCreationOptionsTag},
    credentials_container::{CredentialsContainerMethod, CredentialsContainerTag},
    public_key_credential_creation_options::PublicKeyCredentialCreationOptions,
    Method,
};
use octant_object::define_class;
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
        (**self).invoke(Method::CredentialCreationOptionsMethod(
            self.typed_handle(),
            method,
        ))
    }
}
