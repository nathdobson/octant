use octant_gui_core::{Method, RequestInitMethod, RequestInitTag};
use octant_object::define_class;

use crate::handle::HandleValue;
use crate::object::{Object, ObjectValue};
use crate::runtime::{HasLocalType, HasTypedHandle};

define_class! {
    #[derive(Debug)]
    pub class RequestInit extends Object {
    }
}

impl HasTypedHandle for RequestInitValue {
    type TypeTag = RequestInitTag;
}

impl HasLocalType for RequestInitTag {
    type Local = dyn RequestInit;
}

impl RequestInitValue {
    pub fn new(handle: HandleValue) -> Self {
        RequestInitValue {
            parent: ObjectValue::new(handle),
        }
    }
    fn invoke(&self, method: RequestInitMethod) -> HandleValue {
        (**self).invoke(Method::RequestInit(self.typed_handle(), method))
    }
}
