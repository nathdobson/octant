use crate::{
    js_value::{JsValue, RcJsValue},
    node::{Node, NodeFields},
    object::{Object, ObjectFields},
    octant_runtime::peer::AsNative,
};
use marshal_pointer::RcfRef;
use octant_object::{class, DebugClass};
use octant_runtime::{rpc, runtime::Runtime, DeserializePeer, PeerNew, SerializePeer};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
#[cfg(side="server")]
use crate::global::Global;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct HistoryFields {
    parent: ObjectFields,
    #[cfg(side = "client")]
    wasm: web_sys::History,
    #[cfg(side = "server")]
    state: RefCell<Option<RcJsValue>>,
}

#[class]
pub trait History: Object {
    #[cfg(side = "server")]
    fn replace_state(self: &RcfRef<Self>, state: &RcfRef<dyn JsValue>, url: String) {
        *self.state.borrow_mut() = Some(state.strong());
        self.replace_state_impl(state.strong(), url)
    }
    #[cfg(side = "server")]
    fn state(self: &RcfRef<Self>, global: &Rc<Global>) -> RcJsValue {
        self.state
            .borrow_mut()
            .get_or_insert_with(|| global.null().strong()).clone()
    }
}

#[rpc]
impl dyn History {
    #[rpc]
    fn replace_state_impl(self: &RcfRef<Self>, _: &Rc<Runtime>, state: RcJsValue, url: String) {
        self.native().replace_state(&self.native().state()?, &url)?;
        Ok(())
    }
}
