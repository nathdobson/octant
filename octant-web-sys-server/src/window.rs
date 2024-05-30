use std::{
    future::Future,
    mem::{ManuallyDrop, MaybeUninit},
    rc::Rc,
};

use futures::future::BoxFuture;
use safe_once::cell::OnceCell;
use serde::{de::DeserializeSeed, Deserialize, Deserializer, Serialize, Serializer};

use octant_error::OctantError;
use octant_object::{class, DebugClass};
use octant_reffed::rc::{Rc2, RcRef};
use octant_runtime::{
    define_sys_rpc, future_return::FutureReturn, octant_future::OctantFuture, peer::AsNative,
    runtime::Runtime, DeserializePeer, PeerNew, SerializePeer,
};
#[cfg(side = "client")]
use wasm_bindgen::JsCast;
#[cfg(side = "client")]
use wasm_bindgen_futures::JsFuture;

use crate::{
    document::{Document, DocumentValue, RcDocument},
    navigator::{Navigator, NavigatorValue, RcNavigator},
    object::Object,
    request::{RcRequest, Request},
    response::{RcResponse, ResponseValue},
};
use crate::object::ObjectValue;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct WindowValue {
    parent: ObjectValue,
    #[cfg(side = "client")]
    any_value: web_sys::Window,
    #[cfg(side = "server")]
    document: OnceCell<RcDocument>,
    #[cfg(side = "server")]
    navigator: OnceCell<RcNavigator>,
}

#[class]
pub trait Window: Object {}

#[cfg(side = "server")]
impl dyn Window {
    pub fn fetch<'a>(
        self: &'a RcRef<Self>,
        request: RcRequest,
    ) -> impl 'a + Future<Output = anyhow::Result<RcResponse>> {
        async move { Ok(fetch_wrap(self.runtime(), self.rc(), request).await??) }
    }
    pub fn document<'a>(self: &'a RcRef<Self>) -> &'a RcRef<dyn Document> {
        self.window()
            .document
            .get_or_init(|| document(self.runtime(), self.rc()))
    }
    pub fn navigator<'a>(self: &'a RcRef<Self>) -> &'a RcRef<dyn Navigator> {
        self.window()
            .navigator
            .get_or_init(|| navigator(self.runtime(), self.rc()))
    }
    pub fn alert(self: &RcRef<Self>, message: String) {
        alert(self.runtime(), self.rc(), message);
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
        Ok(Rc2::new(DocumentValue::peer_new(window.native().document().unwrap())))
    }
    fn navigator(_runtime:_,window:RcWindow)->RcNavigator{
        Ok(Rc2::new(NavigatorValue::peer_new(window.native().navigator())))
    }
    fn fetch(runtime: _, window:RcWindow, req:RcRequest) -> OctantFuture<Result<RcResponse, OctantError>>{
        let fetch = window.native().fetch_with_request(req.native());
        Ok(OctantFuture::spawn(runtime, async move{
            Ok(Rc2::new(ResponseValue::peer_new(JsFuture::from(fetch).await.map_err(OctantError::from)?.dyn_into().map_err(OctantError::from)?)) as RcResponse)
        }))
    }
}
