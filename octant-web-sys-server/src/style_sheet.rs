use marshal_pointer::RcfRef;
use octant_object::{class, DebugClass};
use octant_runtime::{DeserializePeer, PeerNew, SerializePeer};
use crate::object::{Object, ObjectFields};

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct StyleSheetFields {
    parent: ObjectFields,
    #[cfg(side = "client")]
    document: web_sys::StyleSheet,
}

#[class]
pub trait StyleSheet: Object {

}
