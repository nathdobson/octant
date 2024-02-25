use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use atomic_refcell::AtomicRefCell;

use octant_gui_core::html_form_element::{
    HtmlFormElementMethod, HtmlFormElementTag, HtmlFormElementUpMessage,
};
use octant_gui_core::Method;
use octant_object::define_class;

use crate::runtime::{HasLocalType, HasTypedHandle};
use crate::{handle, html_element};

struct State {
    handler: Option<Arc<dyn 'static + Sync + Send + Fn()>>,
}

define_class! {
    #[derive(Debug)]
    pub class extends html_element{
        state:AtomicRefCell<State>,
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("handler", &self.handler.is_some())
            .finish()
    }
}

impl HasTypedHandle for Value {
    type TypeTag = HtmlFormElementTag;
}

impl HasLocalType for HtmlFormElementTag {
    type Local = dyn Trait;
}

impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: html_element::Value::new(handle),
            state: AtomicRefCell::new(State { handler: None }),
        }
    }
    fn invoke(&self, method: HtmlFormElementMethod) -> handle::Value {
        (**self).invoke(Method::HtmlFormElement(self.typed_handle(), method))
    }
    pub fn set_handler(&self, callback: impl 'static + Sync + Send + Fn()) {
        self.state.borrow_mut().handler = Some(Arc::new(callback));
        self.invoke(HtmlFormElementMethod::SetListener);
    }
    pub fn submit(&self) {
        self.invoke(HtmlFormElementMethod::Enable);
        if let Some(handler) = self.state.borrow_mut().handler.clone() {
            handler();
        }
    }
    pub fn handle_event(&self, message: HtmlFormElementUpMessage) {
        match message {
            HtmlFormElementUpMessage::Submit => self.submit(),
        }
    }
}
