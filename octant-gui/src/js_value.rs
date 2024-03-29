use octant_object::define_class;

use crate::handle;

define_class! {
    #[derive(Debug)]
    pub class extends handle {
    }
}
impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value { parent: handle }
    }
}
