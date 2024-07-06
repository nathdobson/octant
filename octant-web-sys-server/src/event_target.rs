use crate::{
    event_listener::{EventListenerFields, RcEventListener},
    object::{Object, ObjectFields},
};
use marshal_pointer::{Rcf, RcfRef};
use octant_object::{class, DebugClass};
use octant_runtime::{rpc, runtime::Runtime, DeserializePeer, PeerNew, SerializePeer};
use std::{cell::RefCell, rc::Rc};

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct EventTargetFields {
    parent: ObjectFields,
    #[cfg(side = "client")]
    event_target: web_sys::EventTarget,
    #[cfg(side = "client")]
    listeners: RefCell<Vec<(String, RcEventListener)>>,
    #[cfg(side = "server")]
    listeners: RefCell<Vec<RcEventListener>>,
}

#[class]
pub trait EventTarget: Object {
    #[cfg(side = "server")]
    fn add_listener(self: &RcfRef<Self>, typ: &str, listener: RcEventListener) {
        self.listeners.borrow_mut().push(listener.clone());
        self.add_listener_impl(typ.to_string(), listener)
    }
}

#[rpc]
impl dyn EventTarget {
    #[rpc]
    fn add_listener_impl(
        self: &RcfRef<Self>,
        runtime: &Rc<Runtime>,
        typ: String,
        listener: RcEventListener,
    ) {
        self.event_target
            .add_event_listener_with_callback(&typ, listener.unchecked_ref())?;
        self.listeners.borrow_mut().push((typ, listener.clone()));
        Ok(())
    }
}

#[cfg(side = "client")]
impl Drop for EventTargetFields {
    fn drop(&mut self) {
        for (typ, listener) in self.listeners.get_mut().drain(..) {
            self.event_target
                .remove_event_listener_with_callback(&typ, listener.unchecked_ref())
                .unwrap()
        }
    }
}
