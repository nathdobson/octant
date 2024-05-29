use octant_reffed::arc::ArcRef;
#[cfg(side = "client")]
use octant_runtime::runtime::RuntimeSink;
use octant_runtime::{define_sys_class, peer::Peer, proto::UpMessage, runtime::Runtime};
use octant_serde::{define_serde_impl, DeserializeWith};
use safe_once::sync::OnceLock;
use serde::Serialize;
use std::{
    any::{type_name, Any},
    fmt::{Debug, Formatter},
    sync::Arc,
};

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

define_sys_class! {
    class EventListener;
    extends Peer;
    new_client _;
    new_server _;
    server_field handler: OnceLock<EventHandler>;
}

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
    pub fn fire(self: &ArcRef<Self>) {
        self.sink().send(Box::<EventFired>::new(EventFired {
            listener: self.arc(),
        }))
    }
}

#[derive(Serialize, Debug, DeserializeWith)]
struct EventFired {
    listener: ArcEventListener,
}

define_serde_impl!(EventFired : UpMessage);
impl UpMessage for EventFired {
    #[cfg(side = "server")]
    fn run(self: Box<Self>, runtime: &Arc<Runtime>) -> anyhow::Result<()> {
        if let Some(handler) = self.listener.handler.try_get() {
            (handler.0)()
        }
        Ok(())
    }
}
