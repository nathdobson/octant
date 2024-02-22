#![deny(unused_must_use)]

use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

use anyhow::anyhow;
use atomic_refcell::AtomicRefCell;
use futures::{Sink, Stream, StreamExt};
use futures::SinkExt;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::closure::Closure;
use web_sys::{console, window};
use web_sys::{Document, HtmlFormElement};
use web_sys::{Element, InputEvent};
use web_sys::Node;
use web_sys::Window;

use octant_gui_core::{
    DownMessage, DownMessageList, HandleId, Method, RemoteEvent, TypedHandle, TypeTag,
};
use octant_gui_core::document::{DocumentMethod, DocumentTag};
use octant_gui_core::element::{ElementMethod, ElementTag};
use octant_gui_core::global::GlobalMethod;
use octant_gui_core::html_form_element::{HtmlFormElementMethod, HtmlFormElementTag};
use octant_gui_core::node::NodeTag;
use octant_gui_core::window::{WindowMethod, WindowTag};
use wasm_error::WasmError;

pub type RenderSource = Pin<Box<dyn Stream<Item=anyhow::Result<DownMessageList>>>>;
pub type EventSink = Pin<Box<dyn Sink<RemoteEvent, Error=anyhow::Error>>>;

struct State {
    source: Option<RenderSource>,
    sink: EventSink,
    handles: HashMap<HandleId, JsValue>,
}

pub struct Renderer(AtomicRefCell<State>);

pub trait HasLocalType: TypeTag {
    type Local: JsCast;
}

impl HasLocalType for WindowTag {
    type Local = Window;
}

impl HasLocalType for DocumentTag {
    type Local = Document;
}

impl HasLocalType for ElementTag {
    type Local = Element;
}

impl HasLocalType for NodeTag {
    type Local = Node;
}

impl HasLocalType for HtmlFormElementTag {
    type Local = HtmlFormElement;
}

impl Renderer {
    pub fn new(source: RenderSource, sink: EventSink) -> Arc<Renderer> {
        Arc::new(Renderer(AtomicRefCell::new(State {
            source: Some(source),
            sink,
            handles: HashMap::new(),
        })))
    }
    fn invoke(self: &Arc<Self>, assign: Option<HandleId>, method: &Method) -> anyhow::Result<()> {
        //let ref mut this = *self.0.borrow_mut();
        // let arguments = arguments
        //     .into_iter()
        //     .map(|x| {
        //         Ok(match x {
        //             Argument::Handle(handle) => this
        //                 .handles
        //                 .get(handle)
        //                 .ok_or_else(|| anyhow!("unknown handle"))?
        //                 .clone(),
        //             Argument::Json(json) => {
        //                 serde_wasm_bindgen::to_value(json).map_err(|e| WasmError::new(e.into()))?
        //             }
        //         })
        //     })
        //     .collect::<anyhow::Result<Vec<_>>>()?;
        let result = self.invoke_with(method)?;
        if let Some(assign) = assign {
            console::info_2(&format!("{:?} = ", assign).into(), &result);
            self.0.borrow_mut().handles.insert(assign, result);
        }
        Ok(())
    }
    fn handle<T: HasLocalType>(&self, handle: TypedHandle<T>) -> T::Local {
        self.0
            .borrow()
            .handles
            .get(&handle.0)
            .expect("unknown handle")
            .clone()
            .dyn_into()
            .expect("unexpected type")
    }
    fn invoke_with(self: &Arc<Self>, method: &Method) -> anyhow::Result<JsValue> {
        Ok(match method {
            Method::Global(method) => match method {
                GlobalMethod::Window => window().into(),
            },
            Method::Window(window, method) => {
                let window = self.handle(*window);
                match method {
                    WindowMethod::Document => window.document().into(),
                }
            }
            Method::Document(document, method) => {
                let document = self.handle(*document);
                match method {
                    DocumentMethod::Body => document.body().into(),
                    DocumentMethod::CreateTextNode(text) => document.create_text_node(text).into(),
                    DocumentMethod::CreateElement(tag) => document
                        .create_element(&tag)
                        .map_err(WasmError::new)?
                        .into(),
                    DocumentMethod::CreateFormElement => {
                        let form: HtmlFormElement = document
                            .create_element("form")
                            .map_err(WasmError::new)?
                            .dyn_into()
                            .map_err(|_| anyhow!("expected HtmlFormElement"))?;
                        form.into()
                    }
                }
            }
            Method::Log => {
                todo!()
                // console::info_1(&arguments[0]);
                // JsValue::NULL
            }
            Method::Element(element, method) => {
                let element = self.handle(*element);
                match method {
                    ElementMethod::AppendChild(node) => {
                        let node = self.handle(*node);
                        element.append_child(&node).map_err(WasmError::new)?;
                        JsValue::NULL
                    }
                    ElementMethod::SetAttribute(name, value) => {
                        element
                            .set_attribute(&name, &value)
                            .map_err(WasmError::new)?;
                        JsValue::NULL
                    }
                }
            }
            Method::HtmlFormElement(element_id, method) => {
                let element_id = *element_id;
                let element = self.handle(element_id);
                match method {
                    HtmlFormElementMethod::SetListener => {
                        let this = self.clone();
                        let closure: Box<dyn FnMut(_)> = Box::new({
                            let form = element.clone();
                            move |e: InputEvent| {
                                console::info_2(&"submitted".to_string().into(), &e);
                                e.prevent_default();
                                let children = form.children();
                                for input in 0..children.length() {
                                    let input = children.item(input).unwrap();
                                    input.set_attribute("disabled", "true").unwrap();
                                }
                                let this = this.clone();
                                prokio::spawn_local(async move {
                                    if let Err(err) = this
                                        .0
                                        .borrow_mut()
                                        .sink
                                        .send(RemoteEvent::Submit(element_id))
                                        .await
                                    {
                                        log::error!("Cannot send event {:?}", err);
                                    }
                                });
                            }
                        });
                        let closure = Closure::wrap(closure);
                        element
                            .add_event_listener_with_callback(
                                "submit",
                                closure.as_ref().unchecked_ref(),
                            )
                            .map_err(WasmError::new)?;
                        closure.forget();
                        JsValue::NULL
                    }
                    HtmlFormElementMethod::Enable => {
                        let form = element.clone();
                        let children = form.children();
                        for input in 0..children.length() {
                            let input = children.item(input).unwrap();
                            input.remove_attribute("disabled").unwrap();
                        }
                        JsValue::NULL
                    }
                }
            }
        })
    }
    fn delete(self: &Arc<Self>, handle: HandleId) {
        self.0.borrow_mut().handles.remove(&handle);
    }
    pub async fn run(self: &Arc<Self>) -> anyhow::Result<()> {
        let mut source = self.0.borrow_mut().source.take().unwrap();
        while let Some(commands) = source.next().await {
            let commands = commands?;
            for command in commands.commands {
                console::info_1(&format!("{:?}", command).into());
                match command {
                    DownMessage::Invoke { assign, method } => {
                        self.invoke(assign, &method)?;
                    }
                    DownMessage::Delete(handle) => self.delete(handle),
                }
            }
        }
        Ok(())
    }
}
