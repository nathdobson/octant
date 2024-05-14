use octant_gui_core::HandleId;
use octant_object::{base, define_class};
use octant_object::base::Base;

define_class! {
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
