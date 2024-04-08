use crate::{handle, object, runtime::HasTypedHandle, CredentialCreationOptions};
use octant_gui_core::{
    credentials_container::{CredentialsContainerMethod, CredentialsContainerTag},
    Method,
};
use octant_object::define_class;
define_class! {
    #[derive(Debug)]
    pub class extends object {

    }
}

impl HasTypedHandle for Value {
    type TypeTag = CredentialsContainerTag;
}

impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: object::Value::new(handle),
        }
    }
    pub fn create_with_options(&self, options: &CredentialCreationOptions) {
        self.invoke(CredentialsContainerMethod::CreateWithOptions(options.typed_handle()));
    }
}

impl Value {
    fn invoke(&self, method: CredentialsContainerMethod) -> handle::Value {
        (**self).invoke(Method::CredentialsContainer(self.typed_handle(), method))
    }
}
