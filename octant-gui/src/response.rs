use octant_gui_core::{
    Method
    , ResponseMethod, ResponseTag,
};
use octant_object::define_class;

use crate::{AnyValue, handle, object, promise, Promise, runtime::{HasLocalType, HasTypedHandle}};

define_class! {
    #[derive(Debug)]
    pub class extends object {
    }
}

impl HasTypedHandle for Value {
    type TypeTag = ResponseTag;
}

impl HasLocalType for ResponseTag {
    type Local = dyn Trait;
}

impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: object::Value::new(handle),
        }
    }
    fn invoke(&self, method: ResponseMethod) -> handle::Value {
        (**self).invoke(Method::Response(self.typed_handle(), method))
    }
    pub async fn text(&self) -> anyhow::Result<AnyValue> {
        let promise: Promise = self
            .runtime()
            .add(promise::Value::new(self.invoke(ResponseMethod::Text())));
        promise.wait();
        let resp = promise.get().await?;
        Ok(resp)
    }
}
