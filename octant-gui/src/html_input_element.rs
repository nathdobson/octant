use std::sync::Arc;

use atomic_refcell::AtomicRefCell;

use octant_gui_core::html_form_element::{
    HtmlFormElementMethod, HtmlFormElementTag, HtmlFormElementUpMessage,
};
use octant_gui_core::html_input_element::{
    HtmlInputElementMethod, HtmlInputElementTag, HtmlInputElementUpMessage,
};
use octant_gui_core::Method;
use octant_object::define_class;

use crate::runtime::{HasLocalType, HasTypedHandle};
use crate::{handle, html_element};

#[derive(Debug)]
struct State {
    value: Arc<String>,
}

define_class! {
    #[derive(Debug)]
    pub class extends html_element{
        state:AtomicRefCell<State>,
    }
}

impl HasTypedHandle for Value {
    type TypeTag = HtmlInputElementTag;
}

impl HasLocalType for HtmlInputElementTag {
    type Local = dyn Trait;
}

impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: html_element::Value::new(handle),
            state: AtomicRefCell::new(State {
                value: Arc::new("".to_string()),
            }),
        }
    }
    fn invoke(&self, method: HtmlInputElementMethod) -> handle::Value {
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
}
