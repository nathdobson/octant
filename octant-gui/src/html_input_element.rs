use std::sync::Arc;

use atomic_refcell::AtomicRefCell;

use octant_gui_core::{
    HtmlInputElementMethod, HtmlInputElementTag, HtmlInputElementUpMessage,
};
use octant_gui_core::Method;
use octant_object::define_class;

use crate::{handle, html_element};
use crate::handle::HandleValue;
use crate::html_element::{HtmlElement, HtmlElementValue};
use crate::runtime::{HasLocalType, HasTypedHandle};

#[derive(Debug)]
struct State {
    value: Arc<String>,
}

define_class! {
    #[derive(Debug)]
    pub class HtmlInputElement extends HtmlElement{
        state: AtomicRefCell<State>,
    }
}

impl HasTypedHandle for HtmlInputElementValue {
    type TypeTag = HtmlInputElementTag;
}

impl HasLocalType for HtmlInputElementTag {
    type Local = dyn HtmlInputElement;
}

impl HtmlInputElementValue {
    pub fn new(handle: HandleValue) -> Self {
        HtmlInputElementValue {
            parent: HtmlElementValue::new(handle),
            state: AtomicRefCell::new(State {
                value: Arc::new("".to_string()),
            }),
        }
    }
    fn invoke(&self, method: HtmlInputElementMethod) -> HandleValue {
        (**self).invoke(Method::HtmlInputElement(self.typed_handle(), method))
    }
    pub fn handle_event(&self, message: HtmlInputElementUpMessage) {
        match message {
            HtmlInputElementUpMessage::SetInput { value } => {
                self.state.borrow_mut().value = Arc::new(value);
            }
        }
    }
    pub fn input_value(&self) -> Arc<String> {
        self.state.borrow_mut().value.clone()
    }
    pub fn clear_value(&self) {
        self.invoke(HtmlInputElementMethod::Clear);
    }
}
