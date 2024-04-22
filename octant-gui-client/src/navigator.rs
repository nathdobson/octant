use std::sync::Arc;

use web_sys::Navigator;

use octant_gui_core::{HandleId, NavigatorMethod};
use octant_gui_core::NavigatorTag;
use octant_object::define_class;

use crate::{credentials_container, HasLocalType, object, peer};

define_class! {
    pub class extends object {
        navigator: Navigator,
    }
}

impl Value {
    pub fn new(handle: HandleId, navigator: Navigator) -> Self {
        Value {
            parent: object::Value::new(handle, navigator.clone().into()),
            navigator,
        }
    }
    pub fn credentials(&self, handle: HandleId) -> Arc<dyn credentials_container::Trait> {
        Arc::new(credentials_container::Value::new(
            handle,
            self.navigator.credentials(),
        ))
    }
    pub fn invoke_with(
        &self,
        method: &NavigatorMethod,
        handle: HandleId,
    ) -> Option<Arc<dyn peer::Trait>> {
        match method {
            NavigatorMethod::Credentials => Some(self.credentials(handle)),
        }
    }
}

impl HasLocalType for NavigatorTag {
    type Local = dyn Trait;
}
