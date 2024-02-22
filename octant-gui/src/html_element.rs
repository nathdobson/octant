use octant_object::define_class;

use crate::{element, handle};

define_class! {
    pub class : element{}
}

impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: element::Value::new(handle),
        }
    }
}
