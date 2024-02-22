use octant_gui_core::node::NodeTag;
use octant_object::define_class;

use crate::{handle, object};
use crate::runtime::HasTypedHandle;

define_class! {
    pub class : object {
    }
}

impl HasTypedHandle for Value {
    type TypeTag = NodeTag;
}

impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: object::Value::new(handle),
        }
    }
}
