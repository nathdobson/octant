use std::sync::Arc;

use octant_gui_core::{HandleId, WindowMethod, WindowTag};
use octant_object::define_class;

use crate::{
    document::{ArcDocument, DocumentValue},
    navigator::{ArcNavigator, NavigatorValue},
    object::{Object, ObjectValue},
    peer::ArcPeer,
    promise::{ArcPromise, PromiseValue},
    request::ArcRequest,
    HasLocalType, Runtime,
};

define_class! {
    pub class Window extends Object {
        window: web_sys::Window,
    }
}

impl WindowValue {
    pub fn new(handle: HandleId, window: web_sys::Window) -> Self {
        WindowValue {
            parent: ObjectValue::new(handle, window.clone().into()),
            window,
        }
    }
    pub fn document(&self, handle: HandleId) -> ArcDocument {
        Arc::new(DocumentValue::new(handle, self.window.document().unwrap()))
    }
    pub fn navigator(&self, handle: HandleId) -> ArcNavigator {
        Arc::new(NavigatorValue::new(handle, self.window.navigator()))
    }
    pub fn fetch(
        &self,
        _runtime: &Arc<Runtime>,
        handle: HandleId,
        request: &ArcRequest,
    ) -> ArcPromise {
        Arc::new(PromiseValue::new(
            handle,
            self.window.fetch_with_request(request.native()),
        ))
    }

    pub fn invoke_with(
        &self,
        runtime: &Arc<Runtime>,
        method: &WindowMethod,
        handle: HandleId,
    ) -> Option<ArcPeer> {
        match method {
            WindowMethod::Document => Some(self.document(handle)),
            WindowMethod::Navigator => Some(self.navigator(handle)),
            WindowMethod::Fetch(request) => {
                Some(self.fetch(runtime, handle, &runtime.handle(*request)))
            }
        }
    }
    pub fn native(&self) -> &web_sys::Window {
        &self.window
    }
}

impl HasLocalType for WindowTag {
    type Local = dyn Window;
}
