use octant_gui_core::{CredentialsContainerMethod, CredentialsContainerTag, Method};
use octant_object::define_class;

use crate::{credential_promise, CredentialCreationOptions, CredentialPromise, CredentialRequestOptions, handle, object, runtime::HasTypedHandle};

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
    pub fn create_with_options(&self, options: &CredentialCreationOptions) -> CredentialPromise {
        let promise: CredentialPromise =
            self.runtime()
                .add(credential_promise::Value::new(self.invoke(
                    CredentialsContainerMethod::CreateWithOptions(options.typed_handle()),
                )));
        promise.wait();
        promise
    }
    pub fn get_with_options(&self, options: &CredentialRequestOptions) -> CredentialPromise {
        let promise: CredentialPromise =
            self.runtime()
                .add(credential_promise::Value::new(self.invoke(
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