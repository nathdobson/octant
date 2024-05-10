use octant_gui_core::{
    CredentialData, CredentialsContainerMethod, CredentialsContainerTag, Method,
};
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
    pub async fn create_with_options(
        &self,
        options: &CredentialCreationOptions,
    ) -> anyhow::Result<CredentialData> {
        let promise: Promise = self.runtime().add(promise::Value::new(self.invoke(
            CredentialsContainerMethod::CreateWithOptions(options.typed_handle()),
        )));
        promise.wait();
        let cred = promise.get().await?;
        let cred = cred.downcast_credential();
        let cred = cred.materialize().await;
        Ok(cred)
    }
    pub async fn get_with_options(
        &self,
        options: &CredentialRequestOptions,
    ) -> anyhow::Result<CredentialData> {
        let promise: Promise = self.runtime().add(promise::Value::new(self.invoke(
            CredentialsContainerMethod::GetWithOptions(options.typed_handle()),
        )));
        promise.wait();
        let cred = promise.get().await?;
        let cred = cred.downcast_credential();
        let cred = cred.materialize().await;
        Ok(cred)
    }
}

impl Value {
    fn invoke(&self, method: CredentialsContainerMethod) -> handle::Value {
        (**self).invoke(Method::CredentialsContainer(self.typed_handle(), method))
    }
}
