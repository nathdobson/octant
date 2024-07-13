use crate::{
    js_string::RcJsString,
    node::{Node, NodeFields},
    octant_runtime::peer::AsNative,
};
use marshal_pointer::RcfRef;
use octant_object::{class, DebugClass};
use octant_runtime::{rpc, runtime::Runtime, DeserializePeer, PeerNew, SerializePeer};
use std::rc::Rc;
#[cfg(side = "client")]
use wasm_bindgen::JsCast;
#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct ElementFields {
    parent: NodeFields,
    #[cfg(side = "client")]
    wasm: web_sys::Element,
}
#[class]
pub trait Element: Node {
    #[cfg(side = "server")]
    fn set_inner_html(self: &RcfRef<Self>, value: RcJsString) {
        self.set_inner_html_impl(value);
    }
    #[cfg(side = "server")]
    fn set_id(self: &RcfRef<Self>, value: String) {
        self.set_id_impl(value)
    }
}

#[rpc]
impl dyn Element {
    #[rpc]
    fn set_id_impl(self: &RcfRef<dyn Element>, _: &Rc<Runtime>, value: String) {
        self.native().set_id(&value);
        Ok(())
    }
    #[rpc]
    fn set_inner_html_impl(self: &RcfRef<dyn Element>, _: &Rc<Runtime>, value: RcJsString) -> () {
        self.native()
            .clone()
            .dyn_into::<extras::ExtraElement>()
            .unwrap()
            .set_inner_html_extra(value.native());
        Ok(())
    }
}

#[cfg(side = "client")]
mod extras {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(
            extends = web_sys::Element,
            extends = web_sys::Node,
            extends = web_sys::EventTarget,
            extends = js_sys::Object,
            js_name = Element,
            typescript_type = "Element"
        )]
        pub type ExtraElement;

        #[wasm_bindgen(structural, method, setter, js_class = "Element", js_name = innerHTML)]
        pub fn set_inner_html_extra(this: &ExtraElement, value: &js_sys::JsString);
    }
}
