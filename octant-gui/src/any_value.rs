use octant_gui_core::{
    AnyValueMethod, AnyValueTag, JsClass, Method,
};
use octant_object::define_class;

use crate::credential::{ArcCredential, CredentialValue};
use crate::handle::{Handle, HandleValue};
use crate::response::{ArcResponse, ResponseValue};
use crate::runtime::HasTypedHandle;

define_class! {
    #[derive(Debug)]
    pub class AnyValue extends Handle {
    }
}

impl AnyValueValue {
    pub fn new(handle: HandleValue) -> Self {
        AnyValueValue { parent: handle }
    }
}

impl dyn AnyValue {
    fn invoke(&self, method: AnyValueMethod) -> HandleValue {
        (**self).invoke(Method::AnyValue(self.typed_handle(), method))
    }
    pub fn downcast_credential(&self) -> ArcCredential {
        self.runtime().add(CredentialValue::new(
            self.invoke(AnyValueMethod::Downcast(JsClass::Credential)),
        ))
    }
    pub fn downcast_response(&self) -> ArcResponse {
        self.runtime().add(ResponseValue::new(
            self.invoke(AnyValueMethod::Downcast(JsClass::Response)),
        ))
    }
}

impl HasTypedHandle for AnyValueValue {
    type TypeTag = AnyValueTag;
}
