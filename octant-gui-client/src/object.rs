use web_sys::js_sys::Object;

use octant_gui_core::HandleId;
use octant_object::define_class;

use crate::js_value;

define_class! {
    pub class extends js_value {
        object: Object,
    }
}

impl Value {
    pub fn new(handle: HandleId, object: Object) -> Self {
        Value {
            parent: js_value::Value::new(handle, object.clone().into()),
            object,
        }
    }
}
