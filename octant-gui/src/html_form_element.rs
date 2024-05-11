use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use atomic_refcell::AtomicRefCell;

use octant_gui_core::{
    HtmlFormElementMethod, HtmlFormElementTag, HtmlFormElementUpMessage,
};
use octant_gui_core::Method;
use octant_object::define_class;

use crate::{handle, html_element};
use crate::handle::HandleValue;
use crate::html_element::{HtmlElement, HtmlElementValue};
use crate::runtime::{HasLocalType, HasTypedHandle};

struct State {
    handler: Option<Arc<dyn 'static + Sync + Send + Fn()>>,
}

define_class! {
    #[derive(Debug)]
    pub class HtmlFormElement extends HtmlElement{
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

impl HasTypedHandle for HtmlFormElementValue {
    type TypeTag = HtmlFormElementTag;
}

impl HasLocalType for HtmlFormElementTag {
    type Local = dyn HtmlFormElement;
}

impl HtmlFormElementValue {
    pub fn new(handle: HandleValue) -> Self {
        HtmlFormElementValue {
            parent: HtmlElementValue::new(handle),
            state: AtomicRefCell::new(State { handler: None }),
        }
    }
    fn invoke(&self, method: HtmlFormElementMethod) -> HandleValue {
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
