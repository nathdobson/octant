use safe_once::cell::OnceCell;
use std::{
    any::{type_name, Any},
    fmt::{Debug, Formatter},
    rc::Rc,
};

use octant_object::class;
use serde::Serialize;

use crate::{credential::AsCredential, object::Object};
use octant_reffed::rc::RcRef;
#[cfg(side = "client")]
use octant_runtime::runtime::RuntimeSink;
use octant_runtime::{
    define_sys_class,
    peer::{Peer, PeerValue},
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

#[class]
#[derive(SerializePeer, DeserializePeer)]
pub struct EventListener {
    parent: dyn Peer,
    #[cfg(side = "server")]
    handler: OnceCell<EventHandler>,
}

#[cfg(side = "server")]
impl PeerNew for EventListenerValue {
    type Builder = PeerValue;
    fn peer_new(builder: Self::Builder) -> Self {
        EventListenerValue {
            parent: builder,
            handler: Default::default(),
        }
    }
}

#[cfg(side = "client")]
impl PeerNew for EventListenerValue {
    type Builder = ();
    fn peer_new(builder: Self::Builder) -> Self {
        EventListenerValue {
            parent: PeerValue::new(),
        }
    }
}

pub trait EventListener: AsEventListener {}

impl<T> EventListener for T where T: AsEventListener {}

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
    fn run(self: Box<Self>, runtime: &Rc<Runtime>) -> anyhow::Result<()> {
        if let Some(handler) = self.listener.handler.try_get() {
            (handler.0)()
        }
        Ok(())
    }
}
