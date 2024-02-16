#![deny(unused_must_use)]

use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

use anyhow::anyhow;
use atomic_refcell::AtomicRefCell;
use futures::SinkExt;
use futures::{Sink, Stream, StreamExt};
use octant_gui_core::{
    Argument, Command, CommandList, DocumentMethod, ElementMethod, GlobalMethod, Handle, Method,
    RemoteEvent, WindowMethod,
};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_error::WasmError;
use web_sys::Node;
use web_sys::Window;
use web_sys::{console, window};
use web_sys::{Document, HtmlFormElement};
use web_sys::{Element, InputEvent};

pub type RenderSource = Pin<Box<dyn Stream<Item = anyhow::Result<CommandList>>>>;
pub type EventSink = Pin<Box<dyn Sink<RemoteEvent, Error = anyhow::Error>>>;

struct State {
    source: Option<RenderSource>,
    sink: EventSink,
    handles: HashMap<Handle, JsValue>,
}
pub struct Renderer(AtomicRefCell<State>);

impl Renderer {
    pub fn new(source: RenderSource, sink: EventSink) -> Arc<Renderer> {
        Arc::new(Renderer(AtomicRefCell::new(State {
            source: Some(source),
            sink,
            handles: HashMap::new(),
        })))
    }
    fn invoke(
        self: &Arc<Self>,
        assign: Option<Handle>,
        method: &Method,
        arguments: &[Argument],
    ) -> anyhow::Result<()> {
        let ref mut this = *self.0.borrow_mut();
        let arguments = arguments
            .into_iter()
            .map(|x| {
                Ok(match x {
                    Argument::Handle(handle) => this
                        .handles
                        .get(handle)
                        .ok_or_else(|| anyhow!("unknown handle"))?
                        .clone(),
                    Argument::Json(json) => {
                        serde_wasm_bindgen::to_value(json).map_err(|e| WasmError::new(e.into()))?
                    }
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        let result = self.invoke_with(method, &arguments)?;
        if let Some(assign) = assign {
            console::info_2(&format!("{:?} = ", assign).into(), &result);
            this.handles.insert(assign, result);
        }
        Ok(())
    }
    fn invoke_with(
        self: &Arc<Self>,
        method: &Method,
        arguments: &[JsValue],
    ) -> anyhow::Result<JsValue> {
        Ok(match method {
            Method::Global(method) => match method {
                GlobalMethod::Window => window().into(),
            },
            Method::Window(method) => {
                let window: &Window = arguments[0]
                    .dyn_ref()
                    .ok_or_else(|| anyhow!("cast to Window"))?;
                match method {
                    WindowMethod::Document => window.document().into(),
                }
            }
            Method::Document(method) => {
                let document: &Document = arguments[0]
                    .dyn_ref()
                    .ok_or_else(|| anyhow!("cast to Document"))?;
                match method {
                    DocumentMethod::Body => document.body().into(),
                    DocumentMethod::CreateTextNode => {
                        let text = arguments[1]
                            .as_string()
                            .ok_or_else(|| anyhow!("expected string"))?;
                        document.create_text_node(&text).into()
                    }
                    DocumentMethod::CreateElement => {
                        let tag = arguments[1]
                            .as_string()
                            .ok_or_else(|| anyhow!("expected string"))?;
                        document
                            .create_element(&tag)
                            .map_err(WasmError::new)?
                            .into()
                    }
                    DocumentMethod::CreateFormElement => {
                        let form: HtmlFormElement = document
                            .create_element("form")
                            .map_err(WasmError::new)?
                            .dyn_into()
                            .map_err(|_| anyhow!("expected HtmlFormElement"))?;
                        let this = self.clone();
                        let closure: Box<dyn FnMut(_)> = Box::new({
                            let form = form.clone();
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
                                    if let Err(err) =
                                        this.0.borrow_mut().sink.send(RemoteEvent::Submit).await
                                    {
                                        log::error!("Cannot send event {:?}", err);
                                    }
                                });
                            }
                        });
                        let closure = Closure::wrap(closure);
                        form.add_event_listener_with_callback(
                            "submit",
                            closure.as_ref().unchecked_ref(),
                        )
                        .map_err(WasmError::new)?;
                        closure.forget();
                        form.into()
                    }
                }
            }
            Method::Log => {
                console::info_1(&arguments[0]);
                JsValue::NULL
            }
            Method::Element(method) => {
                let element: &Element = arguments[0]
                    .dyn_ref()
                    .ok_or_else(|| anyhow!("cast to Element"))?;
                match method {
                    ElementMethod::AppendChild => {
                        let node: &Node = arguments[1]
                            .dyn_ref()
                            .ok_or_else(|| anyhow!("cast to Node"))?;
                        element.append_child(node).map_err(WasmError::new)?;
                        JsValue::NULL
                    }
                    ElementMethod::SetAttribute => {
                        let name = arguments[1]
                            .as_string()
                            .ok_or_else(|| anyhow!("expected string"))?;
                        let value = arguments[2]
                            .as_string()
                            .ok_or_else(|| anyhow!("expected string"))?;
                        element
                            .set_attribute(&name, &value)
                            .map_err(WasmError::new)?;
                        JsValue::NULL
                    }
                }
            }
        })
    }
    fn delete(self: &Arc<Self>, handle: Handle) {
        self.0.borrow_mut().handles.remove(&handle);
    }
    pub async fn run(self: &Arc<Self>) -> anyhow::Result<()> {
        let mut source = self.0.borrow_mut().source.take().unwrap();
        while let Some(commands) = source.next().await {
            let commands = commands?;
            for command in commands.commands {
                console::info_1(&format!("{:?}", command).into());
                match command {
                    Command::Invoke {
                        assign,
                        method,
                        arguments,
                    } => {
                        self.invoke(assign, &method, &arguments)?;
                    }
                    Command::Delete(handle) => self.delete(handle),
                }
            }
        }
        Ok(())
    }
}
