use octant_gui_core::{define_sys_class, HandleId};

#[cfg(side = "client")]
use octant_gui_client::peer::{Peer as CustomPeer, PeerValue};

#[cfg(side = "server")]
use octant_gui::handle::{Handle as CustomPeer, HandleValue};

define_sys_class! {
    class AnyValue;
    extends CustomPeer;
    wasm wasm_bindgen::JsValue;
}

#[cfg(side = "client")]
impl AnyValueValue {
    pub fn new(handle: HandleId, any_value: wasm_bindgen::JsValue) -> Self {
        AnyValueValue {
            parent: PeerValue::new(handle),
            any_value,
        }
    }
}

#[cfg(side = "server")]
impl AnyValueValue {
    pub fn new(handle: HandleValue) -> Self {
        AnyValueValue { parent: handle }
    }
}
