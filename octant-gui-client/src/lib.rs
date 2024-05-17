#![deny(unused_must_use)]
#![feature(trait_upcasting)]
#![feature(ptr_metadata)]
#![feature(never_type)]
#![feature(trait_alias)]
#![feature(unsize)]

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::Unsize,
    pin::Pin,
    ptr::{DynMetadata, Pointee},
    sync::Arc,
};

use anyhow::anyhow;
use atomic_refcell::AtomicRefCell;
use catalog::{Builder, BuilderFrom, Registry};
use futures::{Stream, StreamExt};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{console, window, Event, HtmlAnchorElement};

use octant_gui_core::{
    DownMessage, DownMessageList, HandleId, Method, NewDownMessage, TypeTag, TypedHandle,
    UpMessage, UpMessageList,
};
use octant_object::cast::downcast_object;
use wasm_error::WasmError;

use crate::peer::{ArcPeer, Peer};

pub mod any_value;
pub mod credential;
pub mod credential_creation_options;
pub mod credential_request_options;
pub mod credentials_container;
pub mod document;
pub mod element;
pub mod export;
pub mod global;
pub mod html_element;
pub mod html_form_element;
pub mod html_input_element;
pub mod import;
pub mod js_value;
pub mod navigator;
pub mod node;
pub mod object;
pub mod peer;
pub mod promise;
pub mod request;
pub mod request_init;
pub mod response;
pub mod text;
pub mod window;

pub type DownMessageStream = Pin<Box<dyn Stream<Item = anyhow::Result<DownMessageList>>>>;
pub type UpMessageSink = Box<dyn Fn(UpMessageList) -> anyhow::Result<()>>;

struct State {
    source: Option<DownMessageStream>,
    handles: HashMap<HandleId, ArcPeer>,
}

pub struct Runtime {
    state: AtomicRefCell<State>,
    sink: UpMessageSink,
}

pub trait HasLocalType: TypeTag {
    type Local: ?Sized + Pointee<Metadata = DynMetadata<Self::Local>>;
}

impl Runtime {
    pub fn new(source: DownMessageStream, sink: UpMessageSink) -> anyhow::Result<Arc<Runtime>> {
        let runtime = Arc::new(Runtime {
            state: AtomicRefCell::new(State {
                source: Some(source),
                handles: HashMap::new(),
            }),
            sink,
        });
        let window = window().unwrap();
        runtime.send(UpMessageList {
            commands: vec![UpMessage::VisitPage(
                window.location().href().map_err(WasmError::new)?,
            )],
        })?;
        let history = window.history().map_err(WasmError::new)?;
        let document = window.document().unwrap();
        let click_listener = Closure::wrap(Box::new({
            let runtime = Arc::downgrade(&runtime);
            move |e: Event| {
                if let Some(element) = e.target() {
                    let element: Result<HtmlAnchorElement, _> = element.dyn_into();
                    if let Ok(element) = element {
                        e.prevent_default();
                        history
                            .push_state_with_url(&JsValue::NULL, "???", Some(&element.href()))
                            .unwrap();
                        if let Some(runtime) = runtime.upgrade() {
                            runtime
                                .send(UpMessageList {
                                    commands: vec![UpMessage::VisitPage(element.href())],
                                })
                                .unwrap();
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        document
            .add_event_listener_with_callback("click", click_listener.as_ref().unchecked_ref())
            .map_err(WasmError::new)?;
        click_listener.forget();

        let pop_listener = Closure::wrap(Box::new({
            let window = window.clone();
            let runtime = Arc::downgrade(&runtime);
            move |_: Event| {
                if let Some(runtime) = runtime.upgrade() {
                    runtime
                        .send(UpMessageList {
                            commands: vec![UpMessage::VisitPage(
                                window.location().href().map_err(WasmError::new).unwrap(),
                            )],
                        })
                        .unwrap();
                }
            }
        }) as Box<dyn FnMut(_)>);
        window
            .add_event_listener_with_callback("popstate", pop_listener.as_ref().unchecked_ref())
            .map_err(WasmError::new)?;
        pop_listener.forget();
        Ok(runtime)
    }
    pub fn add<T: HasLocalType>(self: &Arc<Self>, assign: TypedHandle<T>, value: Arc<T::Local>)
    where
        T::Local: Unsize<dyn Peer>,
    {
        assert!(self
            .state
            .borrow_mut()
            .handles
            .insert(assign.0, value)
            .is_none());
    }
    async fn invoke(self: &Arc<Self>, assign: HandleId, method: &Method) -> anyhow::Result<()> {
        if let Some(result) = self.invoke_with(method, assign).await? {
            self.state.borrow_mut().handles.insert(assign, result);
        }
        Ok(())
    }
    pub fn handle<T: HasLocalType>(&self, handle: TypedHandle<T>) -> Arc<T::Local> {
        downcast_object(
            self.state
                .borrow()
                .handles
                .get(&handle.0)
                .expect("unknown handle")
                .clone(),
        )
        .unwrap_or_else(|_| panic!("Wrong class for {:?}", handle))
    }
    async fn invoke_with(
        self: &Arc<Self>,
        method: &Method,
        handle: HandleId,
    ) -> anyhow::Result<Option<ArcPeer>> {
        Ok(match method {
            Method::Global(method) => global::invoke_with(self, method, handle),
            Method::Window(window, method) => {
                self.handle(*window).invoke_with(self, method, handle)
            }
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
                self.handle(*element_id)
                    .invoke_with(self.clone(), method, handle)
            }
            Method::HtmlInputElement(element, method) => {
                self.handle(*element)
                    .invoke_with(self.clone(), method, handle)
            }
            Method::Node(node, method) => {
                self.handle(*node)
                    .invoke_with(&self.clone(), method, handle)
            }
            Method::Navigator(node, method) => self.handle(*node).invoke_with(method, handle),
            Method::CredentialsContainer(node, method) => {
                self.handle(*node)
                    .invoke_with(&self.clone(), method, handle)
                    .await
            }
            Method::CredentialCreationOptions(node, method) => {
                self.handle(*node).invoke_with(method, handle)
            }
            Method::CredentialRequestOptions(node, method) => {
                self.handle(*node).invoke_with(method, handle)
            }
            Method::Promise(node, method) => {
                self.handle(*node)
                    .invoke_with(&self.clone(), method, handle)
            }
            Method::AnyValue(node, method) => {
                self.handle(*node)
                    .invoke_with(&self.clone(), method, handle)
            }
            Method::Credential(node, method) => {
                self.handle(*node)
                    .invoke_with(&self.clone(), method, handle)
            }
            Method::Request(node, method) => {
                self.handle(*node)
                    .invoke_with(&self.clone(), method, handle)
            }
            Method::RequestInit(node, method) => {
                self.handle(*node)
                    .invoke_with(&self.clone(), method, handle)
            }
            Method::Response(node, method) => {
                self.handle(*node)
                    .invoke_with(&self.clone(), method, handle)
            }
        })
    }
    fn delete(self: &Arc<Self>, handle: HandleId) {
        self.state.borrow_mut().handles.remove(&handle);
    }
    pub async fn run(self: &Arc<Self>) -> anyhow::Result<!> {
        let mut source = self.state.borrow_mut().source.take().unwrap();
        while let Some(commands) = source.next().await {
            let commands = commands?;
            for command in commands.commands {
                console::info_1(&format!("{:?}", command).into());
                match command {
                    DownMessage::Invoke { assign, method } => {
                        self.invoke(assign, &method).await?;
                    }
                    DownMessage::Delete(handle) => self.delete(handle),
                    DownMessage::Fail(msg) => return Err(anyhow::Error::msg(msg)),
                    DownMessage::NewDownMessage(message) => self.invoke_new(message).await?,
                }
            }
        }
        Err(anyhow!("Websocket terminated."))
    }
    async fn invoke_new(self: &Arc<Self>, message: Box<dyn NewDownMessage>) -> anyhow::Result<()> {
        let handler = DOWN_MESSAGE_HANDLER_REGISTRY
            .handlers
            .get(&(&*message as &dyn Any).type_id())
            .ok_or_else(|| anyhow!("Missing handler for {:?}", message))?;
        handler(ClientContext { runtime: self }, message)?;
        Ok(())
    }
    pub fn send(&self, message: UpMessageList) -> anyhow::Result<()> {
        (self.sink)(message)
    }
}

pub struct ClientContext<'a> {
    pub runtime: &'a Arc<Runtime>,
}

type DynDownMessageHandler = Box<
    dyn 'static
        + Send
        + Sync
        + for<'a> Fn(ClientContext<'a>, Box<dyn NewDownMessage>) -> anyhow::Result<()>,
>;

pub struct DownMessageHandlerRegistry {
    handlers: HashMap<TypeId, DynDownMessageHandler>,
}

impl Builder for DownMessageHandlerRegistry {
    type Output = Self;
    fn new() -> Self {
        DownMessageHandlerRegistry {
            handlers: HashMap::new(),
        }
    }
    fn build(self) -> Self::Output {
        self
    }
}

impl<T: NewDownMessage> BuilderFrom<DownMessageHandler<T>> for DownMessageHandlerRegistry {
    fn insert(&mut self, handler: DownMessageHandler<T>) {
        self.handlers.insert(
            TypeId::of::<T>(),
            Box::new(move |ctx, message| {
                handler(
                    ctx,
                    *Box::<dyn Any>::downcast(message as Box<dyn Any>)
                        .ok()
                        .unwrap(),
                )
            }),
        );
    }
}

pub static DOWN_MESSAGE_HANDLER_REGISTRY: Registry<DownMessageHandlerRegistry> = Registry::new();

pub type DownMessageHandler<T> = for<'a> fn(ClientContext<'a>, T) -> anyhow::Result<()>;
