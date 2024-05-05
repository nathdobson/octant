use std::sync::Arc;

use web_sys::console::log_1;
use web_sys::CredentialsContainer;

use octant_gui_core::{CredentialsContainerMethod, CredentialsContainerTag, HandleId};
use octant_object::define_class;

use crate::{HasLocalType, object, peer, promise, Runtime};

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
                log_1(&options.native());
                Some(Arc::new(promise::Value::new(
                    handle,
                    self.credentials_container
                        .create_with_options(options.native())
                        .unwrap(),
                )))
            }
            CredentialsContainerMethod::GetWithOptions(options) => {
                let options = runtime.handle(*options);
                Some(Arc::new(promise::Value::new(
                    handle,
                    self.credentials_container
                        .get_with_options(options.native())
                        .unwrap(),
                )))
            }
        }
    }
}

impl HasLocalType for CredentialsContainerTag {
    type Local = dyn Trait;
}
