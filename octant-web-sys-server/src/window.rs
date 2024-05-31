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
    document::{Document, DocumentFields, RcDocument},
    navigator::{Navigator, NavigatorFields, RcNavigator},
    object::{Object, ObjectFields},
    request::{RcRequest, Request},
    response::{RcResponse, ResponseFields},
};

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct WindowFields {
    parent: ObjectFields,
    #[cfg(side = "client")]
    any_value: web_sys::Window,
    #[cfg(side = "server")]
    document: OnceCell<RcDocument>,
    #[cfg(side = "server")]
    navigator: OnceCell<RcNavigator>,
}

#[cfg(side = "server")]
pub type FetchFuture<'a> = impl 'a + Future<Output = anyhow::Result<RcResponse>>;

#[class]
pub trait Window: Object {
    #[cfg(side = "server")]
    fn fetch<'a>(self: &'a RcRef<Self>, request: RcRequest) -> FetchFuture<'a> {
        async move { Ok(fetch_wrap(self.runtime(), self.rc(), request).await??) }
    }
    #[cfg(side = "server")]
    fn document<'a>(self: &'a RcRef<Self>) -> &'a RcRef<dyn Document> {
        self.document
            .get_or_init(|| document(self.runtime(), self.rc()))
    }
    #[cfg(side = "server")]
    fn navigator<'a>(self: &'a RcRef<Self>) -> &'a RcRef<dyn Navigator> {
        self.navigator
            .get_or_init(|| navigator(self.runtime(), self.rc()))
    }
    #[cfg(side = "server")]
    fn alert(self: &RcRef<Self>, message: String) {
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
        Ok(Rc2::new(DocumentFields::peer_new(window.native().document().unwrap())))
    }
    fn navigator(_runtime:_,window:RcWindow)->RcNavigator{
        Ok(Rc2::new(NavigatorFields::peer_new(window.native().navigator())))
    }
    fn fetch(runtime: _, window:RcWindow, req:RcRequest) -> OctantFuture<Result<RcResponse, OctantError>>{
        let fetch = window.native().fetch_with_request(req.native());
        Ok(OctantFuture::spawn(runtime, async move{
            Ok(Rc2::new(ResponseFields::peer_new(JsFuture::from(fetch).await.map_err(OctantError::from)?.dyn_into().map_err(OctantError::from)?)) as RcResponse)
        }))
    }
}
