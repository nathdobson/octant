use octant_gui_core::{
    AnyValueMethod, AnyValueTag, JsClass, Method,
};
use octant_object::define_class;

use crate::{credential, Credential, handle, Response, response, runtime::HasTypedHandle};

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

impl dyn Trait {
    fn invoke(&self, method: AnyValueMethod) -> handle::Value {
        (**self).invoke(Method::AnyValue(self.typed_handle(), method))
    }
    pub fn downcast_credential(&self) -> Credential {
        self.runtime().add(credential::Value::new(
            self.invoke(AnyValueMethod::Downcast(JsClass::Credential)),
        ))
    }
    pub fn downcast_response(&self) -> Response {
        self.runtime().add(response::Value::new(
            self.invoke(AnyValueMethod::Downcast(JsClass::Response)),
        ))
    }
}

impl HasTypedHandle for Value {
    type TypeTag = AnyValueTag;
}
