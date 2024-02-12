#![deny(unused_must_use)]

use std::collections::HashMap;
use std::pin::Pin;

use anyhow::anyhow;
use futures::{Stream, StreamExt};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, window};
use web_sys::Document;
use web_sys::Element;
use web_sys::Node;
use web_sys::Window;

use octant_gui_core::{Argument, Command, CommandList, DocumentMethod, ElementMethod, GlobalMethod, Handle, Method, WindowMethod};
use wasm_error::WasmError;

pub type RenderSource = Pin<Box<dyn Stream<Item=anyhow::Result<CommandList>>>>;

pub struct Renderer {
    source: RenderSource,
    handles: HashMap<Handle, JsValue>,
}

impl Renderer {
    pub fn new(source: RenderSource) -> Renderer {
        Renderer { source, handles: HashMap::new() }
    }
    fn invoke(&mut self, assign: Option<Handle>, method: &Method, arguments: &[Argument]) -> anyhow::Result<()> {
        let arguments = arguments.into_iter().map(|x| Ok(match x {
            Argument::Handle(handle) => {
                self.handles.get(handle).ok_or_else(|| anyhow!("unknown handle"))?.clone()
            }
            Argument::Json(json) => {
                serde_wasm_bindgen::to_value(json).map_err(|e| WasmError::new(e.into()))?
            }
        })).collect::<anyhow::Result<Vec<_>>>()?;
        let result = self.invoke_with(method, &arguments)?;
        if let Some(assign) = assign {
            console::info_2(&format!("{:?} = ", assign).into(), &result);
            self.handles.insert(assign, result);
        }
        Ok(())
    }
    fn invoke_with(&mut self, method: &Method, arguments: &[JsValue]) -> anyhow::Result<JsValue> {
        Ok(match method {
            Method::Global(method) => {
                match method {
                    GlobalMethod::Window => window().into(),
                }
            }
            Method::Window(method) => {
                let window: &Window = arguments[0].dyn_ref().ok_or_else(|| anyhow!("cast to Window"))?;
                match method {
                    WindowMethod::Document => window.document().into()
                }
            }
            Method::Document(method) => {
                let document: &Document = arguments[0].dyn_ref().ok_or_else(|| anyhow!("cast to Document"))?;
                match method {
                    DocumentMethod::Body => document.body().into(),
                    DocumentMethod::CreateTextNode => {
                        let text = arguments[1].as_string().ok_or_else(|| anyhow!("expected string"))?;
                        document.create_text_node(&text).into()
                    }
                }
            }
            Method::Log => {
                console::info_1(&arguments[0]);
                JsValue::NULL
            }
            Method::Element(method) => {
                let element: &Element = arguments[0].dyn_ref().ok_or_else(|| anyhow!("cast to Element"))?;
                match method {
                    ElementMethod::AppendChild => {
                        let node: &Node = arguments[1].dyn_ref().ok_or_else(|| anyhow!("cast to Node"))?;
                        element.append_child(node).map_err(WasmError::new)?;
                        JsValue::NULL
                    }
                }
            }
        })
    }
    fn delete(&mut self, handle: Handle) {
        self.handles.remove(&handle);
    }
    pub async fn run(&mut self) -> anyhow::Result<()> {
        while let Some(commands) = self.source.next().await {
            let commands = commands?;
            for command in commands.commands {
                console::info_1(&format!("{:?}", command).into());
                match command {
                    Command::Invoke { assign, method, arguments } => {
                        self.invoke(assign, &method, &arguments)?;
                    }
                    Command::Delete(handle) => {
                        self.delete(handle)
                    }
                }
            }
        }
        Ok(())
    }
}