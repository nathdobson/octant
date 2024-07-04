use marshal_pointer::rc_ref::RcRef;
use safe_once::cell::OnceCell;
use std::{fmt::Debug, rc::Rc};
use marshal_pointer::rcf::Rcf;
use crate::{
    event_listener::RcEventListener,
    html_element::{HtmlElement, HtmlElementFields},
    html_input_element::RcHtmlInputElement,
    node::Node,
    object::Object,
    octant_runtime::peer::AsNative,
};
use octant_object::{cast::downcast_object, class, DebugClass};
use octant_runtime::{rpc, runtime::Runtime, DeserializePeer, PeerNew, SerializePeer};
#[cfg(side = "client")]
use wasm_bindgen::closure::Closure;
#[cfg(side = "client")]
use wasm_bindgen::JsCast;
#[cfg(side = "client")]
use web_sys::Event;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct HtmlFormElementFields {
    parent: HtmlElementFields,
    #[cfg(side = "client")]
    any_value: web_sys::HtmlFormElement,
    #[cfg(side = "client")]
    closure: OnceCell<Closure<dyn Fn(Event)>>,
    #[cfg(side = "server")]
    listener: OnceCell<RcEventListener>,
}

#[class]
pub trait HtmlFormElement: HtmlElement {
    #[cfg(side = "server")]
    fn set_listener(self: &RcRef<Self>, listener: RcEventListener) {
        self.html_form_element()
            .listener
            .get_or_init(|| listener.clone());
        self.set_listener_impl(listener);
    }
}

#[rpc]
impl dyn HtmlFormElement {
    #[rpc]
    fn set_listener_impl(self: &RcRef<Self>, runtime: &Rc<Runtime>, listener: RcEventListener) {
        let cb = Closure::<dyn Fn(Event)>::new({
            let listener = Rcf::downgrade(&listener);
            let this = Rcf::downgrade(&self.rcf());
            move |e: Event| {
                e.prevent_default();
                if let Some(this) = this.upgrade() {
                    for child in this.children() {
                        if let Ok(child) = downcast_object::<_, RcHtmlInputElement>(child) {
                            child.update_value();
                        }
                    }
                }
                if let Some(listener) = listener.upgrade() {
                    listener.fire();
                }
            }
        });
        self.native()
            .add_event_listener_with_callback("submit", cb.as_ref().unchecked_ref())
            .unwrap();
        self.html_form_element().closure.get_or_init(|| cb);
        Ok(())
    }
}
