use std::sync::Arc;

use web_sys::Window;

use octant_gui_core::HandleId;
use octant_gui_core::window::{WindowMethod, WindowTag};
use octant_object::define_class;

use crate::{document, HasLocalType, object, peer};

define_class! {
    pub class extends object {
        window: Window,
    }
}

impl Value {
    pub fn new(handle: HandleId, window: Window) -> Self {
        Value {
            parent: object::Value::new(handle, window.clone().into()),
            window,
        }
    }
    pub fn document(&self, handle: HandleId) -> Arc<dyn document::Trait> {
        Arc::new(document::Value::new(
            handle,
            self.window.document().unwrap(),
        ))
    }

    pub fn invoke_with(&self, method: &WindowMethod, handle: HandleId) -> Option<Arc<dyn peer::Trait>> {
        match method {
            WindowMethod::Document => Some(self.document(handle)),
        }
    }
}

impl HasLocalType for WindowTag {
    type Local = dyn Trait;
}
