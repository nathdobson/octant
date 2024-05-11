use octant_object::define_class;

use crate::handle;
use crate::handle::{Handle, HandleValue};

define_class! {
    #[derive(Debug)]
    pub class JsValue extends Handle {
    }
}
impl JsValueValue {
    pub fn new(handle: HandleValue) -> Self { JsValueValue { parent: handle }
    }
}
