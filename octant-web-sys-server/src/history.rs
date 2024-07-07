#[cfg(side = "server")]
use crate::global::Global;
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

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct HistoryFields {
    parent: ObjectFields,
    #[cfg(side = "client")]
    wasm: web_sys::History,
}

#[class]
pub trait History: Object {
    // #[cfg(side = "server")]
    // fn replace_state(self: &RcfRef<Self>, url: String) {
    //     self.replace_state_impl(url)
    // }
    // #[cfg(side = "server")]
    // fn push_state(self: &RcfRef<Self>, url: String) {
    //     self.push_state_impl(url)
    // }
}

#[rpc]
impl dyn History {
    #[rpc]
    pub fn replace_state(self: &RcfRef<Self>, _: &Rc<Runtime>, title: String, url: Option<String>) {
        self.native()
            .replace_state_with_url(&wasm_bindgen::JsValue::null(), &title, url.as_deref())?;
        Ok(())
    }
    #[rpc]
    pub fn push_state(self: &RcfRef<Self>, _: &Rc<Runtime>, title: String, url: Option<String>) {
        self.native()
            .push_state_with_url(&wasm_bindgen::JsValue::null(), &title, url.as_deref())?;
        Ok(())
    }
}
