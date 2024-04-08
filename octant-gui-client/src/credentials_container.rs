use std::sync::Arc;
use wasm_bindgen_futures::JsFuture;

use octant_gui_core::HandleId;
use octant_object::define_class;

use crate::{object, peer, HasLocalType, Runtime};
use octant_gui_core::{
    credentials_container::{CredentialsContainerMethod, CredentialsContainerTag},
    navigator::NavigatorTag,
};
use web_sys::CredentialsContainer;

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
                JsFuture::from(
                    self.credentials_container
                        .create_with_options(options.native())
                        .unwrap(),
                )
                .await
                .unwrap();
                None
            }
        }
    }
}

impl HasLocalType for CredentialsContainerTag {
    type Local = dyn Trait;
}
