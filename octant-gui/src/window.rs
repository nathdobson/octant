use std::sync::OnceLock;

use octant_gui_core::{Method, WindowMethod, WindowTag};
use octant_object::define_class;

use crate::{document, Document, handle, navigator, Navigator, node, promise, Promise, Request, Response, runtime::HasTypedHandle};

define_class! {
    #[derive(Debug)]
    pub class extends node {
        document: OnceLock<Document>,
        navigator: OnceLock<Navigator>,
    }
}

impl HasTypedHandle for Value {
    type TypeTag = WindowTag;
}

impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: node::Value::new(handle),
            document: OnceLock::new(),
            navigator: OnceLock::new(),
        }
    }
}

impl Value {
    fn invoke(&self, method: WindowMethod) -> handle::Value {
        (**self).invoke(Method::Window(self.typed_handle(), method))
    }
    pub fn document(&self) -> &Document {
        self.document.get_or_init(|| {
            self.runtime()
                .add(document::Value::new(self.invoke(WindowMethod::Document)))
        })
    }
    pub fn navigator(&self) -> &Navigator {
        self.navigator.get_or_init(|| {
            self.runtime()
                .add(navigator::Value::new(self.invoke(WindowMethod::Navigator)))
        })
    }
    pub async fn fetch(&self, request: &Request) -> anyhow::Result<Response> {
        let promise: Promise = self.runtime().add(promise::Value::new(
            self.invoke(WindowMethod::Fetch(request.typed_handle())),
        ));
        promise.wait();
        let resp = promise.get().await?;
        let resp = resp.downcast_response();
        Ok(resp)
    }
}
