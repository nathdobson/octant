use futures::future::BoxFuture;
use std::{
    future::Future,
    mem::{ManuallyDrop, MaybeUninit},
    sync::Arc,
};
use std::rc::Rc;

use crate::document::DocumentValue;
use octant_error::OctantError;
use octant_reffed::arc::{Arc2, ArcRef};
use octant_runtime::{
    define_sys_class, define_sys_rpc, future_return::FutureReturn, octant_future::OctantFuture,
    runtime::Runtime,
};
use safe_once::sync::OnceLock;
use serde::{de::DeserializeSeed, Deserialize, Deserializer, Serialize, Serializer};
use octant_reffed::rc::{Rc2, RcRef};
#[cfg(side = "client")]
use wasm_bindgen::JsCast;
#[cfg(side = "client")]
use wasm_bindgen_futures::JsFuture;

use crate::{
    document::{RcDocument, Document},
    navigator::{RcNavigator, Navigator, NavigatorValue},
    object::Object,
    request::{RcRequest, Request},
    response::{RcResponse, ResponseValue},
};

define_sys_class! {
    class Window;
    extends Object;
    wasm web_sys::Window;
    new_client _;
    new_server _;
    server_field document : OnceLock<RcDocument>;
    server_field navigator : OnceLock<RcNavigator>;
    server_fn {
        fn document<'a>(self: &'a RcRef<Self>) -> &'a RcRef<dyn Document> {
            self.window().document.get_or_init(|| document(self.runtime(), self.rc()))
        }
        fn navigator<'a>(self: &'a RcRef<Self>) -> &'a RcRef< dyn Navigator> {
            self.window().navigator.get_or_init(|| navigator(self.runtime(),self.rc()))
        }
        fn alert(self: & RcRef<Self>, message: String) {
            alert(self.runtime(), self.rc(), message);
        }
    }
}

#[cfg(side = "server")]
impl dyn Window {
    pub fn fetch<'a>(
        self: &'a RcRef<Self>,
        request: RcRequest,
    ) -> impl 'a + Future<Output = anyhow::Result<RcResponse>> {
        async move { Ok(fetch_wrap(self.runtime(), self.rc(), request).await??) }
    }
}

#[cfg(side = "server")]
fn fetch_wrap(
    runtime: &Rc<Runtime>,
    window: RcWindow,
    request: RcRequest,
) -> impl Future<Output = Result<Result<RcResponse, OctantError>, anyhow::Error>> {
    fetch(runtime, window, request)
}

define_sys_rpc! {
    fn alert(_runtime:_, window: RcWindow, message: String) -> () {
        window.native().alert_with_message(&message).unwrap();
        Ok(())
    }
    fn document(_runtime:_, window: RcWindow) -> RcDocument {
        Ok(Rc2::new(DocumentValue::new(window.native().document().unwrap())))
    }
    fn navigator(_runtime:_,window:RcWindow)->RcNavigator{
        Ok(Rc2::new(NavigatorValue::new(window.native().navigator())))
    }
    fn fetch(runtime: _, window:RcWindow, req:RcRequest) -> OctantFuture<Result<RcResponse, OctantError>>{
        let fetch = window.native().fetch_with_request(req.native());
        Ok(OctantFuture::spawn(runtime, async move{
            Ok(Rc2::new(ResponseValue::new(JsFuture::from(fetch).await.map_err(OctantError::from)?.dyn_into().map_err(OctantError::from)?)) as RcResponse)
        }))
    }
}
