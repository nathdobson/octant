use std::marker::PhantomData;
use std::sync::Arc;
use wasm_bindgen::closure::Closure;
use web_sys::{console, HtmlFormElement, InputEvent};

use crate::{html_element, peer, HasLocalType, Runtime};
use octant_gui_core::html_form_element::{HtmlFormElementMethod, HtmlFormElementTag};
use octant_gui_core::{HandleId, TypedHandle, UpMessage, UpMessageList};
use octant_object::define_class;
use wasm_bindgen::JsCast;
use wasm_error::WasmError;

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
        handle_id: HandleId,
    ) -> Option<Arc<dyn peer::Trait>> {
        match method {
            HtmlFormElementMethod::SetListener => {
                let this = self.clone();
                let closure: Box<dyn FnMut(_)> = Box::new({
                    let form = self.clone();
                    move |e: InputEvent| {
                        console::info_2(&"submitted".to_string().into(), &e);
                        e.prevent_default();
                        let children = form.native().children();
                        for input in 0..children.length() {
                            let input = children.item(input).unwrap();
                            input.set_attribute("disabled", "true").unwrap();
                        }
                        let runtime = runtime.clone();
                        let this = this.clone();
                        let messages = UpMessageList {
                            commands: vec![UpMessage::Submit(this.handle())],
                        };
                        if let Err(err) = runtime.send(messages) {
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
