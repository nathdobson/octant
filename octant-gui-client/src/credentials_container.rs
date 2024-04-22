use std::sync::Arc;

use web_sys::CredentialsContainer;

use octant_gui_core::{CredentialsContainerMethod, CredentialsContainerTag};
use octant_gui_core::HandleId;
use octant_object::define_class;

use crate::{credential_promise, HasLocalType, object, peer, Runtime};

define_class! {
    pub class extends object {
        credentials_container: CredentialsContainer,
    }
}

impl Value {
    pub fn new(handle: HandleId, credentials_container: CredentialsContainer) -> Self {
        Value {
            parent: object::Value::new(handle, credentials_container.clone().into()),
            credentials_container,
        }
    }
    pub async fn invoke_with(
        &self,
        runtime: &Arc<Runtime>,
        method: &CredentialsContainerMethod,
        handle: HandleId,
    ) -> Option<Arc<dyn peer::Trait>> {
        match method {
            CredentialsContainerMethod::CreateWithOptions(options) => {
                let options = runtime.handle(*options);
                Some(Arc::new(credential_promise::Value::new(
                    handle,
                    self.credentials_container
                        .create_with_options(options.native())
                        .unwrap(),
                )))
            }
        }
    }
}

impl HasLocalType for CredentialsContainerTag {
    type Local = dyn Trait;
}
