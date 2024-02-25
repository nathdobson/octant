use octant_gui_core::HandleId;
use octant_object::{base, define_class};

define_class! {
    pub class extends base {
        handle: HandleId,
    }
}

impl Value {
    pub fn new(handle: HandleId) -> Self {
        Value {
            parent: base::Value::new(),
            handle,
        }
    }
    pub fn raw_handle(&self) -> HandleId {
        self.handle
    }
}
