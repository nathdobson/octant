use safe_once::cell::OnceCell;
use std::{
    any::{type_name, Any},
    fmt::{Debug, Formatter},
    rc::Rc,
};

use octant_object::{class, DebugClass};
use serde::Serialize;
use octant_error::OctantResult;

use crate::object::Object;
use octant_reffed::rc::RcRef;
#[cfg(side = "client")]
use octant_runtime::runtime::RuntimeSink;
use octant_runtime::{
    peer::{Peer, PeerFields},
    proto::UpMessage,
    runtime::Runtime,
    DeserializePeer, PeerNew, SerializePeer,
};
use octant_serde::{define_serde_impl, DeserializeWith};

#[cfg(side = "server")]
trait EventHandlerTrait: 'static + Fn() -> () {
    fn debug_name(&self) -> &'static str {
        type_name::<Self>()
    }
}
#[cfg(side = "server")]
impl<T: 'static + Any + Fn() -> ()> EventHandlerTrait for T {}

#[cfg(side = "server")]
struct EventHandler(Box<dyn EventHandlerTrait>);

#[cfg(side = "server")]
impl Debug for EventHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Handler")
            .field(&self.0.debug_name())
            .finish()
    }
}

#[cfg(side = "server")]
impl EventHandler {
    pub fn new<F: 'static + Fn() -> ()>(f: F) -> Self {
        EventHandler(Box::<F>::new(f))
    }
}

#[derive(DebugClass, SerializePeer, DeserializePeer)]
pub struct EventListenerFields {
    parent: PeerFields,
    #[cfg(side = "server")]
    handler: OnceCell<EventHandler>,
}

#[cfg(side = "server")]
impl PeerNew for EventListenerFields {
    type Builder = PeerFields;
    fn peer_new(builder: Self::Builder) -> Self {
        EventListenerFields {
            parent: builder,
            handler: Default::default(),
        }
    }
}

#[cfg(side = "client")]
impl PeerNew for EventListenerFields {
    type Builder = ();
    fn peer_new(builder: Self::Builder) -> Self {
        EventListenerFields {
            parent: PeerFields::new(),
        }
    }
}

#[class] pub trait EventListener: Peer {}

#[cfg(side = "server")]
impl dyn EventListener {
    pub fn set_handler(&self, handler: impl Any + Fn()) {
        self.event_listener()
            .handler
            .lock()
            .or_init(|| EventHandler::new(handler));
    }
}

#[cfg(side = "client")]
impl dyn EventListener {
    pub fn fire(self: &RcRef<Self>) {
        self.sink().send(Box::<EventFired>::new(EventFired {
            listener: self.rc(),
        }))
    }
}

#[derive(Serialize, Debug, DeserializeWith)]
struct EventFired {
    listener: RcEventListener,
}

define_serde_impl!(EventFired : UpMessage);
impl UpMessage for EventFired {
    #[cfg(side = "server")]
    fn run(self: Box<Self>, runtime: &Rc<Runtime>) -> OctantResult<()> {
        if let Some(handler) = self.listener.handler.try_get() {
            (handler.0)()
        }
        Ok(())
    }
}
