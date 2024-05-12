use std::sync::Arc;

use web_sys::console::log_1;

use octant_gui_core::{CredentialsContainerMethod, CredentialsContainerTag, HandleId};
use octant_object::define_class;

use crate::{HasLocalType, Runtime};
use crate::object::{Object, ObjectValue};
use crate::peer::ArcPeer;
use crate::promise::PromiseValue;

define_class! {
    pub class CredentialsContainer extends Object {
        credentials_container: web_sys::CredentialsContainer,
    }
}

impl CredentialsContainerValue {
    pub fn new(handle: HandleId, credentials_container: web_sys::CredentialsContainer) -> Self {
        CredentialsContainerValue {
            parent: ObjectValue::new(handle, credentials_container.clone().into()),
            credentials_container,
        }
    }
    pub async fn invoke_with(
        &self,
        runtime: &Arc<Runtime>,
        method: &CredentialsContainerMethod,
        handle: HandleId,
    ) -> Option<ArcPeer> {
        match method {
            CredentialsContainerMethod::CreateWithOptions(options) => {
                let options = runtime.handle(*options);
                log_1(&options.native());
                Some(Arc::new(PromiseValue::new(
                    handle,
                    self.credentials_container
                        .create_with_options(options.native())
                        .unwrap(),
                )))
            }
            CredentialsContainerMethod::GetWithOptions(options) => {
                let options = runtime.handle(*options);
                Some(Arc::new(PromiseValue::new(
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
    type Local = dyn CredentialsContainer;
}
