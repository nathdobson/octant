use crate::{
    history::RcHistory,
    html_element::{HtmlElement, HtmlElementFields},
    octant_runtime::peer::AsNative,
};
use marshal_pointer::RcfRef;
use octant_object::{class, DebugClass};
use octant_runtime::{rpc, runtime::Runtime, DeserializePeer, PeerNew, SerializePeer};
use safe_once::cell::OnceCell;
use std::{cell::RefCell, rc::Rc};
#[cfg(side = "client")]
use wasm_bindgen::closure::Closure;
#[cfg(side = "client")]
use wasm_bindgen::JsValue;
#[cfg(side = "client")]
use web_sys::window;
#[cfg(side = "client")]
use web_sys::Event;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct HtmlAnchorElementFields {
    parent: HtmlElementFields,
    #[cfg(side = "client")]
    wasm: web_sys::HtmlAnchorElement,
    href: RefCell<String>,
    #[cfg(side = "client")]
    history: OnceCell<RcHistory>,
}

#[class]
pub trait HtmlAnchorElement: HtmlElement {}

#[rpc]
impl dyn HtmlAnchorElement {
    #[rpc]
    pub fn set_href(self: &RcfRef<Self>, _: &Rc<Runtime>, href: String) -> () {
        self.native().set_attribute("href", &href)?;
        *self.href.borrow_mut() = href;
        Ok(())
    }
    #[rpc]
    pub fn set_push_state_handler(self: &RcfRef<Self>, runtime: &Rc<Runtime>, history: RcHistory) {
        let this = self.weak();
        let history = history.weak();
        self.add_listener(
            "click",
            Closure::new(move |e: Event| {
                e.prevent_default();
                if let (Some(this),Some(history)) = (this.upgrade(),history.upgrade()) {
                    history.push_state(&this.href.borrow()).unwrap();
                }
            }),
        )?;
        Ok(())
    }
}
