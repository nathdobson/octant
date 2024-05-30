use crate::credential::AsCredential;
use octant_object::class;
use octant_runtime::{DeserializePeer, PeerNew, SerializePeer};

use crate::{node::Node, object::Object};

#[class]
#[derive(PeerNew, SerializePeer, DeserializePeer)]
pub struct Text {
    parent: dyn Node,
    #[cfg(side = "client")]
    any_value: web_sys::Text,
}

pub trait Text: AsText {}

impl<T> Text for T where T: AsText {}
