use octant_object::define_class;

use crate::{handle, js_value};

define_class! {
    pub class extends js_value{
    }
}
impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: js_value::Value::new(handle),
        }
    }
}
