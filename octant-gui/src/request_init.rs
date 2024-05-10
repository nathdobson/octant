use octant_gui_core::{Method, RequestInitMethod, RequestInitTag};
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
    type TypeTag = RequestInitTag;
}

impl HasLocalType for RequestInitTag {
    type Local = dyn Trait;
}

impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: object::Value::new(handle),
        }
    }
    fn invoke(&self, method: RequestInitMethod) -> handle::Value {
        (**self).invoke(Method::RequestInit(self.typed_handle(), method))
    }
}
