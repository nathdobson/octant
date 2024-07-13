use crate::{
    attributes::autocomplete::Autocomplete,
    html_element::{HtmlElement, HtmlElementFields},
    object::Object,
    octant_runtime::peer::AsNative,
};
use marshal::{Deserialize, Serialize};
use marshal_object::derive_variant;
use marshal_pointer::RcfRef;
use octant_error::OctantResult;
use octant_object::{class, DebugClass};
use octant_runtime::{
    proto::{BoxUpMessage, UpMessage},
    rpc,
    runtime::Runtime,
    DeserializePeer, PeerNew, SerializePeer,
};
use std::{cell::RefCell, rc::Rc};
use crate::attributes::input_type::InputType;

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
    fn update_input_value(self: &RcfRef<Self>) {
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

#[rpc]
impl dyn HtmlInputElement {
    #[rpc]
    pub fn set_autocomplete(self: &RcfRef<Self>, _: &Rc<Runtime>, autocomplete: Autocomplete) {
        self.native().set_autocomplete(&autocomplete.to_string());
        Ok(())
    }
    #[rpc]
    pub fn set_type(self: &RcfRef<Self>, _: &Rc<Runtime>, typ: InputType) {
        self.native().set_type(&typ.as_string());
        Ok(())
    }
    #[rpc]
    pub fn set_value(self: &RcfRef<Self>, _: &Rc<Runtime>, value: String) {
        self.native().set_value(&value);
        Ok(())
    }
    #[rpc]
    pub fn set_placeholder(self: &RcfRef<Self>, _: &Rc<Runtime>, placeholder: String) {
        self.native().set_placeholder(&placeholder);
        Ok(())
    }
    #[rpc]
    pub fn set_required(self: &RcfRef<Self>, _: &Rc<Runtime>, required: bool) {
        self.native().set_required(required);
        Ok(())
    }
}
