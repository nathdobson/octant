use octant_object::class;
use octant_runtime::{define_sys_class, DeserializePeer, PeerNew, SerializePeer};

use crate::object::Object;

#[class]
#[derive(PeerNew, SerializePeer, DeserializePeer)]
pub struct Credential {
    parent: dyn Object,
    #[cfg(side = "client")]
    any_value: web_sys::Credential,
}

pub trait Credential: AsCredential {}

impl<T> Credential for T where T: AsCredential {}