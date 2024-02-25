#![deny(unused_must_use)]
#![feature(trait_upcasting)]
#![feature(ptr_metadata)]

use std::collections::HashMap;
use std::pin::Pin;
use std::ptr::{DynMetadata, Pointee};
use std::sync::Arc;

use atomic_refcell::AtomicRefCell;
use futures::SinkExt;
use futures::{Sink, Stream, StreamExt};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::InputEvent;
use web_sys::{console, window};

use octant_gui_core::html_form_element::{HtmlFormElementMethod};
use octant_gui_core::{
    DownMessage, DownMessageList, HandleId, Method, RemoteEvent, TypeTag, TypedHandle,
};
use octant_object::cast::Cast;
use wasm_error::WasmError;

mod document;
mod element;
mod global;
mod html_element;
mod html_form_element;
mod js_value;
mod node;
mod object;
mod peer;
mod text;
mod window;

pub type RenderSource = Pin<Box<dyn Stream<Item = anyhow::Result<DownMessageList>>>>;
pub type EventSink = Pin<Box<dyn Sink<RemoteEvent, Error = anyhow::Error>>>;

struct State {
    source: Option<RenderSource>,
    sink: EventSink,
    handles: HashMap<HandleId, Arc<dyn peer::Trait>>,
}

pub struct Runtime(AtomicRefCell<State>);

pub trait HasLocalType: TypeTag {
    type Local: ?Sized + Pointee<Metadata = DynMetadata<Self::Local>>;
}

pub type WindowPeer = Arc<dyn window::Trait>;

impl Runtime {
    pub fn new(source: RenderSource, sink: EventSink) -> Arc<Runtime> {
        Arc::new(Runtime(AtomicRefCell::new(State {
            source: Some(source),
            sink,
            handles: HashMap::new(),
        })))
    }
    fn invoke(self: &Arc<Self>, assign: HandleId, method: &Method) -> anyhow::Result<()> {
        if let Some(result) = self.invoke_with(method, assign)? {
            self.0.borrow_mut().handles.insert(assign, result);
        }
        Ok(())
    }
    fn handle<T: HasLocalType>(&self, handle: TypedHandle<T>) -> Arc<T::Local> {
        self.0
            .borrow()
            .handles
            .get(&handle.0)
            .expect("unknown handle")
            .clone()
            .downcast_trait()
            .unwrap()
    }
    fn invoke_with(
        self: &Arc<Self>,
        method: &Method,
        handle: HandleId,
    ) -> anyhow::Result<Option<Arc<dyn peer::Trait>>> {
        Ok(match method {
            Method::Global(method) => global::invoke_with(method, handle),
            Method::Window(window, method) => self.handle(*window).invoke_with(method, handle),
            Method::Document(document, method) => {
                self.handle(*document).handle_with(method, handle)
            }
            Method::Log => {
                todo!()
            }
            Method::Element(element, method) => {
                self.handle(*element).invoke_with(self, method, handle)
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
                                let children = form.native().children();
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
                            .native()
                            .add_event_listener_with_callback(
                                "submit",
                                closure.as_ref().unchecked_ref(),
                            )
                            .map_err(WasmError::new)?;
                        closure.forget();
                        None
                    }
                    HtmlFormElementMethod::Enable => {
                        let form = element.clone();
                        let children = form.native().children();
                        for input in 0..children.length() {
                            let input = children.item(input).unwrap();
                            input.remove_attribute("disabled").unwrap();
                        }
                        None
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
