use crate::credential::AsCredential;
use octant_object::class;
use octant_runtime::{define_sys_class, DeserializePeer, PeerNew, SerializePeer};

use crate::object::Object;

#[class]
#[derive(PeerNew, SerializePeer, DeserializePeer)]
pub struct Request {
    parent: dyn Object,
    #[cfg(side = "client")]
    any_value: web_sys::Request,
}

pub trait Request: AsRequest {}

impl<T> Request for T where T: AsRequest {}
