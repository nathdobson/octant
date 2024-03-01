use std::marker::PhantomData;
use std::sync::Arc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlFormElement, InputEvent};

use octant_gui_core::{HandleId, TypedHandle, UpMessage, UpMessageList};
use octant_gui_core::html_form_element::{HtmlFormElementMethod, HtmlFormElementTag, HtmlFormElementUpMessage};
use octant_gui_core::html_input_element::HtmlInputElementUpMessage;
use octant_object::cast::Cast;
use octant_object::define_class;

use crate::{HasLocalType, html_element, html_input_element, peer, Runtime};

define_class! {
    pub class extends html_element {
        html_form_element: HtmlFormElement,
    }
}

impl Value {
    pub fn new(handle: HandleId, html_form_element: HtmlFormElement) -> Self {
        Value {
            parent: html_element::Value::new(handle, html_form_element.clone().into()),
            html_form_element,
        }
    }
    pub fn handle(&self) -> TypedHandle<HtmlFormElementTag> {
        TypedHandle(self.raw_handle(), PhantomData)
    }
}
impl dyn Trait {
    pub fn invoke_with(
        self: Arc<Self>,
        runtime: Arc<Runtime>,
        method: &HtmlFormElementMethod,
        _handle_id: HandleId,
    ) -> Option<Arc<dyn peer::Trait>> {
        match method {
            HtmlFormElementMethod::SetListener => {
                let this = self.clone();
                let closure: Box<dyn FnMut(_)> = Box::new({
                    let form = self.clone();
                    move |e: InputEvent| {
                        e.prevent_default();
                        let mut commands: Vec<UpMessage> = vec![];
                        let children = form.children();
                        for child in children {
                            let input: Option<Arc<dyn html_input_element::Trait>> =
                                child.downcast_trait();
                            if let Some(input) = input {
                                input.native().set_attribute("disabled", "true").unwrap();
                                commands.push(UpMessage::HtmlInputElement(
                                    input.handle(),
                                    HtmlInputElementUpMessage::SetInput {
                                        value: input.native().value(),
                                    },
                                ))
                            }
                        }
                        commands.push(UpMessage::HtmlFormElement(this.handle(),HtmlFormElementUpMessage::Submit));
                        if let Err(err) = runtime.send(UpMessageList { commands }) {
                            log::error!("Cannot send event {:?}", err);
                        }
                    }
                });
                let closure = Closure::wrap(closure);
                self.native()
                    .add_event_listener_with_callback("submit", closure.as_ref().unchecked_ref())
                    .unwrap();
                closure.forget();
                None
            }
            HtmlFormElementMethod::Enable => {
                let form = self.clone();
                let children = form.native().children();
                for input in 0..children.length() {
                    let input = children.item(input).unwrap();
                    input.remove_attribute("disabled").unwrap();
                }
                None
            }
        }
    }
}

impl HasLocalType for HtmlFormElementTag {
    type Local = dyn Trait;
}
