use crate::credential::AsCredential;
use octant_object::class;
use octant_runtime::{define_sys_class, DeserializePeer, PeerNew, SerializePeer};

use crate::js_value::JsValue;

#[class]
#[derive(PeerNew, SerializePeer, DeserializePeer)]
pub struct Object {
    parent: dyn JsValue,
    #[cfg(side = "client")]
    any_value: js_sys::Object,
}

pub trait Object: AsObject {}

impl<T> Object for T where T: AsObject {}
