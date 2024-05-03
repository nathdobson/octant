use octant_gui_core::{CredentialsContainerMethod, CredentialsContainerTag, Method};
use octant_object::define_class;

use crate::{
    CredentialCreationOptions, CredentialRequestOptions, handle, object, promise,
    Promise, runtime::HasTypedHandle,
};

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
    pub fn create_with_options(&self, options: &CredentialCreationOptions) -> Promise {
        let promise: Promise = self.runtime().add(promise::Value::new(self.invoke(
            CredentialsContainerMethod::CreateWithOptions(options.typed_handle()),
        )));
        promise.wait();
        promise
    }
    pub fn get_with_options(&self, options: &CredentialRequestOptions) -> Promise {
        let promise: Promise = self.runtime().add(promise::Value::new(self.invoke(
            CredentialsContainerMethod::GetWithOptions(options.typed_handle()),
        )));
        promise.wait();
        promise
    }
}

impl Value {
    fn invoke(&self, method: CredentialsContainerMethod) -> handle::Value {
        (**self).invoke(Method::CredentialsContainer(self.typed_handle(), method))
    }
}
