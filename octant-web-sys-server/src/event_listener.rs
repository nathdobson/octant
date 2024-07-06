#[cfg(side = "server")]
use crate::event_handler::EventHandler;
use crate::object::Object;
#[cfg(side = "client")]
use js_sys::Function;
use marshal::{Deserialize, Serialize};
use marshal_object::derive_variant;
use marshal_pointer::{EmptyRcf, raw_any::RawAny, RcfRef};
use octant_error::OctantResult;
use octant_object::{class, DebugClass};
use octant_runtime::{
    peer::{Peer, PeerFields},
    proto::{BoxUpMessage, UpMessage},
    rpc,
    runtime::Runtime,
    DeserializePeer, PeerNew, SerializePeer,
};
use safe_once::cell::OnceCell;
use std::{
    any::{type_name, Any},
    cell::Cell,
    fmt::{Debug, Formatter},
    rc::Rc,
};
#[cfg(side = "client")]
use wasm_bindgen::closure::Closure;
#[cfg(side = "client")]
use wasm_bindgen::JsCast;
#[cfg(side = "client")]
use wasm_bindgen::JsValue;
#[cfg(side = "client")]
use web_sys::Event;

#[derive(DebugClass, SerializePeer, DeserializePeer)]
pub struct EventListenerFields {
    parent: PeerFields,
    #[cfg(side = "server")]
    handler: OnceCell<Box<dyn EventHandler>>,
    #[cfg(side = "client")]
    handler: Closure<dyn Fn(Event)>,
    #[cfg(side = "client")]
    prevent_default: Cell<bool>,
}

#[class]
pub trait EventListener: Peer {
    #[cfg(side = "client")]
    fn fire(self: &RcfRef<Self>) {
        self.sink().send(Box::<EventFired>::new(EventFired {
            listener: self.strong(),
        }))
    }
    #[cfg(side = "server")]
    fn set_handler(self: &RcfRef<Self>, handler: Box<dyn EventHandler>) {
        self.event_listener().handler.lock().or_init(|| handler);
    }
    #[cfg(side = "client")]
    fn unchecked_ref<'a>(self: &'a RcfRef<Self>) -> &'a Function {
        self.handler.as_ref().unchecked_ref()
    }
    #[cfg(side = "client")]
    fn prevent_default(self: &RcfRef<Self>) -> bool {
        self.prevent_default.get()
    }
}

#[rpc]
impl dyn EventListener {
    #[rpc]
    pub fn set_prevent_default(self: &RcfRef<Self>, runtime: &Rc<Runtime>, prevent_default: bool) {
        self.prevent_default.set(prevent_default);
        Ok(())
    }
}

#[cfg(side = "client")]
impl PeerNew for EventListenerFields {
    type Builder = Closure<dyn Fn(Event)>;
    fn peer_new(builder: Self::Builder) -> Self {
        EventListenerFields {
            parent: PeerFields::new(),
            handler: builder,
            prevent_default: Cell::new(false),
        }
    }
}

#[cfg(side = "server")]
impl PeerNew for EventListenerFields {
    type Builder = PeerFields;
    fn peer_new(builder: Self::Builder) -> Self {
        EventListenerFields {
            parent: builder,
            handler: OnceCell::new(),
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
struct EventFired {
    listener: RcEventListener,
}

derive_variant!(BoxUpMessage, EventFired);

impl UpMessage for EventFired {
    #[cfg(side = "server")]
    fn run(self: Box<Self>, runtime: &Rc<Runtime>) -> OctantResult<()> {
        if let Some(handler) = self.listener.handler.try_get() {
            (handler)()
        }
        Ok(())
    }
}

#[rpc]
pub fn new_event_listener(_: &Rc<Runtime>) -> RcEventListener {
    let this = EmptyRcf::<EventListenerFields>::new();
    let cb = Closure::<dyn Fn(Event)>::new({
        let this = this.downgrade();
        move |e: Event| {
            if let Some(this) = this.upgrade() {
                if this.prevent_default(){
                    e.prevent_default()
                }
                this.fire();
            }
        }
    });
    Ok(this.into_strong(EventListenerFields::peer_new(cb)))
}
