use octant_gui_core::{Method, RequestMethod, RequestTag};
use octant_object::define_class;

use crate::{
    object::Object,
    runtime::{HasLocalType, HasTypedHandle},
};
use crate::handle::HandleValue;
use crate::object::ObjectValue;

define_class! {
    #[derive(Debug)]
    pub class Request extends Object {
    }
}

impl HasTypedHandle for RequestValue {
    type TypeTag = RequestTag;
}

impl HasLocalType for RequestTag {
    type Local = dyn Request;
}

impl RequestValue {
    pub fn new(handle: HandleValue) -> Self {
        RequestValue {
            parent: ObjectValue::new(handle),
        }
    }
    fn invoke(&self, method: RequestMethod) -> HandleValue {
        (**self).invoke(Method::Request(self.typed_handle(), method))
    }
}
