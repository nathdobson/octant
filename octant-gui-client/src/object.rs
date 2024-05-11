
use octant_gui_core::HandleId;
use octant_object::define_class;

use crate::js_value;
use crate::js_value::{JsValue, JsValueValue};

define_class! {
    pub class Object extends JsValue {
        object: js_sys::Object,
    }
}

impl ObjectValue {
    pub fn new(handle: HandleId, object: js_sys::Object) -> Self {
        ObjectValue {
            parent: JsValueValue::new(handle, object.clone().into()),
            object,
        }
    }
}
