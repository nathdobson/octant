use octant_object::class;
use octant_runtime::{define_sys_class, DeserializePeer, PeerNew, SerializePeer};
use crate::credential::AsCredential;

use crate::node::Node;
use crate::object::Object;

#[class]
#[derive(PeerNew, SerializePeer, DeserializePeer)]
pub struct Text {
    parent: dyn Node,
    #[cfg(side = "client")]
    any_value: web_sys::Text,
}

pub trait Text: AsText {}

impl<T> Text for T where T: AsText {}