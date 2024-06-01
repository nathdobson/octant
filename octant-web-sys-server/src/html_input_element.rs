use std::{cell::RefCell, rc::Rc};

use octant_object::{class, DebugClass};
use serde::Serialize;
use octant_error::OctantResult;

use octant_reffed::rc::RcRef;
use octant_runtime::{
    peer::AsNative, proto::UpMessage, runtime::Runtime, DeserializePeer, PeerNew, SerializePeer,
};
use octant_serde::{define_serde_impl, DeserializeWith};

use crate::{event_listener::RcEventListener, html_element::HtmlElement, object::Object};
use crate::html_element::HtmlElementFields;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct HtmlInputElementFields {
    parent: HtmlElementFields,
    #[cfg(side = "client")]
    any_value: web_sys::HtmlInputElement,
    #[cfg(side = "server")]
    value: RefCell<Rc<String>>,
}
#[class]
pub trait HtmlInputElement: HtmlElement {
    #[cfg(side = "client")]
    fn update_value(self: &RcRef<Self>) {
        let this = self as &RcRef<dyn crate::html_input_element::HtmlInputElement>;
        this.sink().send(Box::<SetInput>::new(SetInput {
            element: self.rc(),
            value: this.native().value(),
        }));
    }
    #[cfg(side = "server")]
    fn input_value(&self) -> Rc<String> {
        self.html_input_element().value.borrow_mut().clone()
    }
}

#[derive(Serialize, Debug, DeserializeWith)]
struct SetInput {
    element: RcHtmlInputElement,
    value: String,
}

define_serde_impl!(SetInput : UpMessage);
impl UpMessage for SetInput {
    #[cfg(side = "server")]
    fn run(self: Box<Self>, runtime: &Rc<Runtime>) -> OctantResult<()> {
        *self.element.html_input_element().value.borrow_mut() = Rc::new(self.value);
        Ok(())
    }
}
