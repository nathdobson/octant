use crate::object::{Object, ObjectFields};
#[cfg(side = "client")]
use js_sys::Function;
use marshal_pointer::{Rcf, RcfRef};
use octant_error::OctantResult;
use octant_object::{class, DebugClass};
use octant_runtime::{rpc, runtime::Runtime, DeserializePeer, PeerNew, SerializePeer};
use std::{cell::RefCell, rc::Rc};
#[cfg(side = "client")]
use wasm_bindgen::closure::Closure;
#[cfg(side = "client")]
use wasm_bindgen::JsCast;
#[cfg(side = "client")]
use web_sys::Event;

#[cfg(side = "client")]
#[derive(Debug)]
pub struct ClientEventHandler {
    closure: Closure<dyn Fn(Event)>,
}

#[cfg(side = "client")]
impl ClientEventHandler {
    pub fn new<F: 'static + Fn(Event) -> OctantResult<()>>(f: F) -> Self {
        ClientEventHandler {
            closure: Closure::new(move |e| {
                f(e).unwrap()
            }),
        }
    }
    pub fn unchecked_ref(&self) -> &Function {
        self.closure.as_ref().unchecked_ref()
    }
}

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct EventTargetFields {
    parent: ObjectFields,
    #[cfg(side = "client")]
    event_target: web_sys::EventTarget,
    #[cfg(side = "client")]
    listeners: RefCell<Vec<(String, ClientEventHandler)>>,
}

#[class]
pub trait EventTarget: Object {
    #[cfg(side = "client")]
    fn add_listener(
        self: &RcfRef<Self>,
        typ: &str,
        listener: ClientEventHandler,
    ) -> OctantResult<()> {
        self.event_target
            .add_event_listener_with_callback(&typ, listener.unchecked_ref())?;
        self.listeners.borrow_mut().push((typ.to_owned(), listener));
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
