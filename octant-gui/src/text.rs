use octant_object::define_class;

use crate::{handle, node};

define_class! {
    pub class : node{}
}

impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: node::Value::new(handle),
        }
    }
}
