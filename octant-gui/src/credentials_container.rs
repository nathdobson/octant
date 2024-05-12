use octant_gui_core::{
    CredentialData, CredentialsContainerMethod, CredentialsContainerTag, Method,
};
use octant_object::define_class;

use crate::credential_creation_options::ArcCredentialCreationOptions;
use crate::credential_request_options::ArcCredentialRequestOptions;
use crate::handle::HandleValue;
use crate::object::{Object, ObjectValue};
use crate::promise::{ArcPromise, PromiseValue};
use crate::runtime::HasTypedHandle;

define_class! {
    #[derive(Debug)]
    pub class CredentialsContainer extends Object {

    }
}

impl HasTypedHandle for CredentialsContainerValue {
    type TypeTag = CredentialsContainerTag;
}

impl CredentialsContainerValue {
    pub fn new(handle: HandleValue) -> Self {
        CredentialsContainerValue {
            parent: ObjectValue::new(handle),
        }
    }
    pub async fn create_with_options(
        &self,
        options: &ArcCredentialCreationOptions,
    ) -> anyhow::Result<CredentialData> {
        let promise: ArcPromise = self.runtime().add(PromiseValue::new(self.invoke(
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
        options: &ArcCredentialRequestOptions,
    ) -> anyhow::Result<CredentialData> {
        let promise: ArcPromise = self.runtime().add(PromiseValue::new(self.invoke(
            CredentialsContainerMethod::GetWithOptions(options.typed_handle()),
        )));
        promise.wait();
        let cred = promise.get().await?;
        let cred = cred.downcast_credential();
        let cred = cred.materialize().await;
        Ok(cred)
    }
}

impl CredentialsContainerValue {
    fn invoke(&self, method: CredentialsContainerMethod) -> HandleValue {
        (**self).invoke(Method::CredentialsContainer(self.typed_handle(), method))
    }
}
