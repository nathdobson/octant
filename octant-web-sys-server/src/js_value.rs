use octant_gui_core::{define_sys_class, FromHandle, HandleId, NewTypedHandle};

#[cfg(side = "client")]
use octant_gui_client::peer::{Peer as CustomPeer, PeerValue};

#[cfg(side = "server")]
use octant_gui::handle::{Handle as CustomPeer, HandleValue};

define_sys_class! {
    class JsValue;
    extends CustomPeer;
    wasm wasm_bindgen::JsValue;
}

// #[cfg(side = "client")]
// impl JsValueValue {
//     pub fn new(handle: NewTypedHandle<dyn JsValue>, js_value: wasm_bindgen::JsValue) -> Self {
//         JsValueValue {
//             parent: PeerValue::new(handle.raw()),
//             js_value,
//         }
//     }
// }

#[cfg(side = "server")]
impl JsValueValue {
    pub fn new(handle: HandleValue) -> Self {
        JsValueValue { parent: handle }
    }
}

#[cfg(side = "client")]
impl FromHandle for dyn JsValue {
    type Builder = wasm_bindgen::JsValue;
    fn from_handle(handle: NewTypedHandle<Self>, js_value: Self::Builder) -> Self::Value {
        JsValueValue {
            parent: PeerValue::new(handle.raw()),
            js_value,
        }
    }
}
