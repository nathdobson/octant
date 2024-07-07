#[cfg(side = "server")]
use crate::event_handler::EventHandler;
#[cfg(side = "client")]
use crate::event_target::ClientEventHandler;
use crate::{
    document::{Document, RcDocument},
    event_target::{EventTarget, EventTargetFields},
    history::{History, RcHistory},
    navigator::{Navigator, RcNavigator},
    object::{Object, ObjectFields},
    octant_runtime::peer::AsNative,
    request::{RcRequest, Request},
    response::RcResponse,
};
use marshal::{Deserialize, Serialize};
use marshal_object::derive_variant;
use marshal_pointer::RcfRef;
use octant_error::{OctantError, OctantResult};
use octant_object::{class, DebugClass};
use octant_runtime::{
    future_return::FutureReturn,
    octant_future::OctantFuture,
    proto::{BoxUpMessage, UpMessage},
    rpc,
    runtime::Runtime,
    DeserializePeer, PeerNew, SerializePeer,
};
use safe_once::cell::OnceCell;
use std::{future::Future, rc::Rc};
#[cfg(side = "client")]
use wasm_bindgen::closure::Closure;
#[cfg(side = "client")]
use wasm_bindgen::JsCast;
#[cfg(side = "client")]
use wasm_bindgen_futures::JsFuture;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct WindowFields {
    parent: EventTargetFields,
    #[cfg(side = "client")]
    any_value: web_sys::Window,
    #[cfg(side = "server")]
    document: OnceCell<RcDocument>,
    #[cfg(side = "server")]
    navigator: OnceCell<RcNavigator>,
    #[cfg(side = "server")]
    history: OnceCell<RcHistory>,
    #[cfg(side = "server")]
    pop_state_handler: OnceCell<Box<dyn EventHandler<String>>>,
}

#[cfg(side = "server")]
pub type FetchFuture<'a> = <() as FetchFutureTrait>::Fut<'a>;

#[cfg(side = "server")]
pub trait FetchFutureTrait {
    type Fut<'a>: 'a + Future<Output = OctantResult<RcResponse>>
    where
        Self: 'a;
    fn fetch<'a>(this: &'a RcfRef<dyn Window>, request: RcRequest) -> Self::Fut<'a>;
}

#[cfg(side = "server")]
impl FetchFutureTrait for () {
    type Fut<'a> = impl 'a + Future<Output = OctantResult<RcResponse>>;
    fn fetch<'a>(this: &'a RcfRef<dyn Window>, request: RcRequest) -> Self::Fut<'a> {
        async move { Ok(this.fetch_impl(request).await??) }
    }
}

#[class]
pub trait Window: EventTarget {
    #[cfg(side = "server")]
    fn fetch<'a>(self: &'a RcfRef<Self>, request: RcRequest) -> FetchFuture<'a> {
        <()>::fetch(self, request)
    }
    #[cfg(side = "server")]
    fn document<'a>(self: &'a RcfRef<Self>) -> &'a RcfRef<dyn Document> {
        self.document.get_or_init(|| self.document_impl())
    }
    #[cfg(side = "server")]
    fn navigator<'a>(self: &'a RcfRef<Self>) -> &'a RcfRef<dyn Navigator> {
        self.navigator.get_or_init(|| self.navigator_impl())
    }
    #[cfg(side = "server")]
    fn history<'a>(self: &'a RcfRef<Self>) -> &'a RcfRef<dyn History> {
        self.history.get_or_init(|| self.history_impl())
    }
    #[cfg(side = "server")]
    fn alert(self: &RcfRef<Self>, message: String) {
        self.alert_impl(message);
    }

    #[cfg(side = "server")]
    fn set_pop_state_handler(self: &RcfRef<Self>, handler: Box<dyn EventHandler<String>>) {
        self.pop_state_handler.set(handler).ok().unwrap();
        self.set_pop_state_handler_impl();
    }
}

#[rpc]
impl dyn Window {
    #[rpc]
    fn document_impl(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcDocument {
        Ok(RcDocument::peer_new(self.native().document().unwrap()))
    }
    #[rpc]
    fn alert_impl(self: &RcfRef<Self>, _: &Rc<Runtime>, message: String) -> () {
        self.native().alert_with_message(&message).unwrap();
        Ok(())
    }
    #[rpc]
    fn navigator_impl(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcNavigator {
        Ok(RcNavigator::peer_new(self.native().navigator()))
    }
    #[rpc]
    fn history_impl(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcHistory {
        Ok(RcHistory::peer_new(self.native().history()?))
    }
    #[rpc]
    fn fetch_impl(
        self: &RcfRef<Self>,
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
    #[rpc]
    fn set_pop_state_handler_impl(self: &RcfRef<Self>, runtime: &Rc<Runtime>) {
        let this = self.weak();
        self.add_listener(
            "popstate",
            ClientEventHandler::new(move |e| {
                if let Some(this) = this.upgrade() {
                    this.sink().send(Box::new(PopState {
                        window: this.clone(),
                        url: this
                            .native()
                            .document()
                            .unwrap()
                            .location()
                            .unwrap()
                            .href()
                            .unwrap(),
                    }));
                }
                Ok(())
            }),
        )?;
        Ok(())
    }
}
#[derive(Serialize, Debug, Deserialize)]
struct PopState {
    window: RcWindow,
    url: String,
}

derive_variant!(BoxUpMessage, PopState);

impl UpMessage for PopState {
    #[cfg(side = "server")]
    fn run(self: Box<Self>, runtime: &Rc<Runtime>) -> OctantResult<()> {
        if let Some(handler) = self.window.pop_state_handler.get() {
            (handler)(self.url)?;
        }
        Ok(())
    }
}
