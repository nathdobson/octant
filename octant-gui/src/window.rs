use std::sync::OnceLock;

use octant_gui_core::{Method, WindowMethod, WindowTag};
use octant_object::define_class;

use crate::document::{ArcDocument, DocumentValue};
use crate::handle::HandleValue;
use crate::navigator::{ArcNavigator, NavigatorValue};
use crate::node::{Node, NodeValue};
use crate::promise::{ArcPromise, PromiseValue};
use crate::request::ArcRequest;
use crate::response::ArcResponse;
use crate::runtime::HasTypedHandle;

define_class! {
    #[derive(Debug)]
    pub class Window extends Node {
        document: OnceLock<ArcDocument>,
        navigator: OnceLock<ArcNavigator>,
    }
}

impl HasTypedHandle for WindowValue {
    type TypeTag = WindowTag;
}

impl WindowValue {
    pub fn new(handle: HandleValue) -> Self {
        WindowValue {
            parent: NodeValue::new(handle),
            document: OnceLock::new(),
            navigator: OnceLock::new(),
        }
    }
}

impl WindowValue {
    fn invoke(&self, method: WindowMethod) -> HandleValue {
        (**self).invoke(Method::Window(self.typed_handle(), method))
    }
    pub fn document(&self) -> &ArcDocument {
        self.document.get_or_init(|| {
            self.runtime()
                .add(DocumentValue::new(self.invoke(WindowMethod::Document)))
        })
    }
    pub fn navigator(&self) -> &ArcNavigator {
        self.navigator.get_or_init(|| {
            self.runtime()
                .add(NavigatorValue::new(self.invoke(WindowMethod::Navigator)))
        })
    }
    pub async fn fetch(&self, request: &ArcRequest) -> anyhow::Result<ArcResponse> {
        let promise: ArcPromise = self.runtime().add(PromiseValue::new(
            self.invoke(WindowMethod::Fetch(request.typed_handle())),
        ));
        promise.wait();
        let resp = promise.get().await?;
        let resp = resp.downcast_response();
        Ok(resp)
    }
}
