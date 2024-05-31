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
    future_return::FutureReturn, octant_future::OctantFuture, peer::AsNative, rpc,
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
        async move { Ok(self.fetch_impl(request).await??) }
    }
    #[cfg(side = "server")]
    fn document<'a>(self: &'a RcRef<Self>) -> &'a RcRef<dyn Document> {
        self.document.get_or_init(|| self.document_impl())
    }
    #[cfg(side = "server")]
    fn navigator<'a>(self: &'a RcRef<Self>) -> &'a RcRef<dyn Navigator> {
        self.navigator.get_or_init(|| self.navigator_impl())
    }
    #[cfg(side = "server")]
    fn alert(self: &RcRef<Self>, message: String) {
        self.alert_impl(message);
    }
}

#[rpc]
impl dyn Window {
    #[rpc]
    fn document_impl(self: &RcRef<Self>, _: &Rc<Runtime>) -> RcDocument {
        Ok(RcDocument::peer_new(self.native().document().unwrap()))
    }
    #[rpc]
    fn alert_impl(self: &RcRef<Self>, _: &Rc<Runtime>, message: String) -> () {
        self.native().alert_with_message(&message).unwrap();
        Ok(())
    }
    #[rpc]
    fn navigator_impl(self: &RcRef<Self>, _: &Rc<Runtime>) -> RcNavigator {
        Ok(RcNavigator::peer_new(self.native().navigator()))
    }
    #[rpc]
    fn fetch_impl(
        self: &RcRef<Self>,
        runtime: &Rc<Runtime>,
        req: RcRequest,
    ) -> OctantFuture<Result<RcResponse, OctantError>> {
        let fetch = self.native().fetch_with_request(req.native());
        Ok(OctantFuture::spawn(runtime, async move {
            Ok(RcResponse::peer_new(
                JsFuture::from(fetch)
                    .await
                    .map_err(OctantError::from)?
                    .dyn_into()
                    .map_err(OctantError::from)?,
            ))
        }))
    }
}
