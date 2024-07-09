use crate::{
    object::{Object, ObjectFields},
    style_sheet::{StyleSheet, StyleSheetFields},
};
use marshal_pointer::RcfRef;
use octant_object::{class, DebugClass};
use octant_runtime::{rpc, runtime::Runtime, DeserializePeer, PeerNew, SerializePeer};
use std::rc::Rc;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct CssStyleSheetFields {
    parent: StyleSheetFields,
    #[cfg(side = "client")]
    native: web_sys::CssStyleSheet,
}

#[class]
pub trait CssStyleSheet: StyleSheet {}

#[rpc]
impl dyn CssStyleSheet {
    #[rpc]
    pub fn insert_rule(self: &RcfRef<Self>, _: &Rc<Runtime>, rule: String) {
        self.native.insert_rule(&rule)?;
        Ok(())
    }
}
