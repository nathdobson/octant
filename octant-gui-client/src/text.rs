use web_sys::Text;

use octant_gui_core::HandleId;
use octant_object::define_class;

use crate::node;

define_class! {
    pub class extends node {
        text:Text,
    }
}

impl Value {
    pub fn new(handle: HandleId, text: Text) -> Self {
        Value {
            parent: node::Value::new(handle, text.clone().into()),
            text,
        }
    }
}
