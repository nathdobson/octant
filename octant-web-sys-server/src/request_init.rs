use octant_object::class;
use octant_runtime::{define_sys_class, DeserializePeer, PeerNew, SerializePeer};

use crate::object::Object;

#[class]
#[derive(PeerNew, SerializePeer, DeserializePeer)]
pub struct RequestInit {
    parent: dyn Object,
    #[cfg(side = "client")]
    any_value: web_sys::RequestInit,
}

pub trait RequestInit: AsRequestInit {}

impl<T> RequestInit for T where T: AsRequestInit {}
