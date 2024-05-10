use octant_gui_core::{Method, RequestMethod, RequestTag};
use octant_object::define_class;

use crate::{
    handle, object,
    runtime::{HasLocalType, HasTypedHandle},
};

define_class! {
    #[derive(Debug)]
    pub class extends object {
    }
}

impl HasTypedHandle for Value {
    type TypeTag = RequestTag;
}

impl HasLocalType for RequestTag {
    type Local = dyn Trait;
}

impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: object::Value::new(handle),
        }
    }
    fn invoke(&self, method: RequestMethod) -> handle::Value {
        (**self).invoke(Method::Request(self.typed_handle(), method))
    }
}
