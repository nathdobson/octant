use std::fmt::Debug;

use octant_gui_core::HandleId;
use octant_object::{base, base::Base, define_class};

define_class! {
    #[derive(Debug)]
    pub class Peer extends Base {
        handle: HandleId,
    }
}

impl PeerValue {
    pub fn new(handle: HandleId) -> Self {
        PeerValue {
            parent: base::BaseValue::new(),
            handle,
        }
    }
    pub fn raw_handle(&self) -> HandleId {
        self.handle
    }
}