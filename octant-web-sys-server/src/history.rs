#[cfg(side = "server")]
use crate::event_handler::EventHandler;
#[cfg(side = "server")]
use crate::global::Global;
use crate::{
    js_value::{JsValue, RcJsValue},
    node::{Node, NodeFields},
    object::{Object, ObjectFields},
    octant_runtime::peer::AsNative,
};
use marshal::{Deserialize, Serialize};
use marshal_object::derive_variant;
use marshal_pointer::RcfRef;
use octant_error::OctantResult;
use octant_object::{class, DebugClass};
use octant_runtime::{
    proto::{BoxUpMessage, UpMessage},
    rpc,
    runtime::Runtime,
    DeserializePeer, PeerNew, SerializePeer,
};
use safe_once::cell::OnceCell;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
#[cfg(side = "client")]
use web_sys::window;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct HistoryFields {
    parent: ObjectFields,
    #[cfg(side = "client")]
    wasm: web_sys::History,
    #[cfg(side = "server")]
    handler: OnceCell<Box<dyn EventHandler<String>>>,
}

#[class]
pub trait History: Object {
    #[cfg(side = "server")]
    fn set_push_state_handler(&self, handler: Box<dyn EventHandler<String>>) {
        self.handler.set(handler).ok().unwrap();
    }
    #[cfg(side = "client")]
    fn push_state(self: &RcfRef<Self>, url: &str) -> OctantResult<()> {
        self.native()
            .push_state_with_url(&wasm_bindgen::JsValue::null(), "", Some(url))
            .unwrap();
        let url = window().unwrap().document().unwrap().location().unwrap().href()?;
        self.sink().send(Box::new(PushState {
            history: self.strong(),
            url,
        }));
        Ok(())
    }
}

#[derive(Serialize, Debug, Deserialize)]
struct PushState {
    history: RcHistory,
    url: String,
}

derive_variant!(BoxUpMessage, PushState);

impl UpMessage for PushState {
    #[cfg(side = "server")]
    fn run(self: Box<Self>, runtime: &Rc<Runtime>) -> OctantResult<()> {
        if let Some(handler) = self.history.handler.get() {
            (handler)(self.url)?;
        }
        Ok(())
    }
}
