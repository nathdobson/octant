use std::cell::RefCell;
use std::rc::Rc;
use marshal::{Deserialize, Serialize};
use marshal_object::derive_variant;
use marshal_pointer::RcfRef;
use octant_error::OctantResult;
use octant_object::{class, DebugClass};
use octant_runtime::{
    DeserializePeer, PeerNew, proto::UpMessage, SerializePeer,
};
use octant_runtime::proto::BoxUpMessage;
use octant_runtime::runtime::Runtime;
use crate::{html_element::HtmlElement, object::Object};
use crate::html_element::HtmlElementFields;
use crate::octant_runtime::peer::AsNative;

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
    fn update_value(self: &RcfRef<Self>) {
        let this = self as &RcfRef<dyn crate::html_input_element::HtmlInputElement>;
        this.sink().send(Box::<SetInput>::new(SetInput {
            element: self.strong(),
            value: this.native().value(),
        }));
    }
    #[cfg(side = "server")]
    fn input_value(&self) -> Rc<String> {
        self.html_input_element().value.borrow_mut().clone()
    }
}

#[derive(Serialize, Debug, Deserialize)]
struct SetInput {
    element: RcHtmlInputElement,
    value: String,
}
derive_variant!(BoxUpMessage, SetInput);

impl UpMessage for SetInput {
    #[cfg(side = "server")]
    fn run(self: Box<Self>, runtime: &Rc<Runtime>) -> OctantResult<()> {
        *self.element.html_input_element().value.borrow_mut() = Rc::new(self.value);
        Ok(())
    }
}
