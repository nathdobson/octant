use octant_object::define_class;

use crate::handle::HandleValue;
use crate::js_value::{JsValue, JsValueValue};

define_class! {
#[derive(Debug)]
        pub class Object extends JsValue{
    }
}

impl ObjectValue {
    pub fn new(handle: HandleValue) -> Self {
        ObjectValue {
            parent: JsValueValue::new(handle),
        }
    }
}
