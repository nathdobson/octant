use octant_gui_core::{Method, ResponseMethod, ResponseTag};
use octant_object::define_class;

use crate::{
    any_value::ArcAnyValue,
    handle::HandleValue,
    object::{Object, ObjectValue},
    promise::{ArcPromise, PromiseValue},
    runtime::{HasLocalType, HasTypedHandle},
};

define_class! {
    #[derive(Debug)]
    pub class Response extends Object {
    }
}

impl HasTypedHandle for ResponseValue {
    type TypeTag = ResponseTag;
}

impl HasLocalType for ResponseTag {
    type Local = dyn Response;
}

impl ResponseValue {
    pub fn new(handle: HandleValue) -> Self {
        ResponseValue {
            parent: ObjectValue::new(handle),
        }
    }
    fn invoke(&self, method: ResponseMethod) -> HandleValue {
        (**self).invoke(Method::Response(self.typed_handle(), method))
    }
    pub async fn text(&self) -> anyhow::Result<ArcAnyValue> {
        let promise: ArcPromise = self
            .runtime()
            .add(PromiseValue::new(self.invoke(ResponseMethod::Text())));
        promise.wait();
        let resp = promise.get().await?;
        Ok(resp)
    }
}
