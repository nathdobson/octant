#![deny(unused_must_use)]
#![feature(trait_upcasting)]
#![feature(ptr_metadata)]
#![feature(never_type)]
#![feature(trait_alias)]
#![feature(unsize)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::{Debug, Formatter},
    marker::Unsize,
    pin::Pin,
    ptr::{DynMetadata, Pointee},
    sync::Arc,
};

use anyhow::anyhow;
use atomic_refcell::AtomicRefCell;
use catalog::{Builder, BuilderFrom, Registry};
use futures::{Stream, StreamExt};
use serde::de::Visitor;
use type_map::TypeMap;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{console, Event, HtmlAnchorElement, window};

use octant_gui_core::{
    FromHandle
    , HandleId, Method, NewTypedHandle, reexports::{
        octant_serde::{define_serde_trait,  DeserializeWith, SerializeDyn},
        serde::{ Deserializer, Serialize},
    }, TypedHandle, TypeTag, UpMessage,
    UpMessageList,
};
use octant_object::{cast::downcast_object, class::Class};
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

pub type DownMessageStream = Pin<Box<dyn Stream<Item = anyhow::Result<ClientDownMessageList>>>>;
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
    pub fn add_new<T: ?Sized + Class + Unsize<dyn Peer>>(
        self: &Arc<Self>,
        assign: NewTypedHandle<T>,
        value: Arc<T>,
    ) {
        assert!(self
            .state
            .borrow_mut()
            .handles
            .insert(assign.raw(), value)
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
    pub fn handle_new<T: ?Sized + Class>(&self, handle: NewTypedHandle<T>) -> Arc<T> {
        downcast_object(
            self.state
                .borrow()
                .handles
                .get(&handle.raw())
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
                self.invoke_new(command).await?;
            }
        }
        Err(anyhow!("Websocket terminated."))
    }
    async fn invoke_new(
        self: &Arc<Self>,
        message: Box<dyn ClientDownMessage>,
    ) -> anyhow::Result<()> {
        let handler = DOWN_MESSAGE_HANDLER_REGISTRY
            .handlers
            .get(&(&*message as &dyn Any).type_id())
            .ok_or_else(|| anyhow!("Missing handler for {:?}", message))?;
        handler(self, message)?;
        Ok(())
    }
    pub fn send(&self, message: UpMessageList) -> anyhow::Result<()> {
        (self.sink)(message)
    }
}

type DynDownMessageHandler = Box<
    dyn 'static
        + Send
        + Sync
        + for<'a> Fn(&'a Arc<Runtime>, Box<dyn ClientDownMessage>) -> anyhow::Result<()>,
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

impl<T: ClientDownMessage> BuilderFrom<DownMessageHandler<T>> for DownMessageHandlerRegistry {
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

pub type DownMessageHandler<T> = for<'a> fn(&'a Arc<Runtime>, T) -> anyhow::Result<()>;

pub struct ReturnValue<T: ?Sized + Class> {
    runtime: Arc<Runtime>,
    handle: NewTypedHandle<T>,
}

impl<T: ?Sized + Class + Unsize<dyn Peer>> ReturnValue<T> {
    pub fn new(runtime: Arc<Runtime>, handle: NewTypedHandle<T>) -> Self {
        ReturnValue { runtime, handle }
    }
    pub fn set_raw(self, value: Arc<T>) {
        self.runtime.add_new(self.handle, value)
    }
    pub fn set(self, builder: T::Builder)
    where
        T: FromHandle,
    {
        let handle = self.handle;
        self.set_raw(Arc::<T::Value>::new(T::from_handle(handle, builder)))
    }
}

// impl<'c, 'de, T: ?Sized + Class> DeserializeArcWith<'de> for T {
//     fn deserialize_arc_with<D: Deserializer<'de>>(ctx: &TypeMap, d: D) -> Result<Arc<T>, D::Error> {
//         let handle = NewTypedHandle::<T>::deserialize(d)?;
//         Ok(ctx.get::<Arc<Runtime>>().unwrap().handle_new(handle))
//     }
// }

pub trait ClientDownMessage: SerializeDyn + Debug + Any {}
define_serde_trait!(ClientDownMessage);

#[derive(Serialize, Debug)]
pub struct ClientDownMessageList {
    pub commands: Vec<Box<dyn ClientDownMessage>>,
}

impl<'de> DeserializeWith<'de> for ClientDownMessageList {
    fn deserialize_with<D: Deserializer<'de>>(ctx: &TypeMap, d: D) -> Result<Self, D::Error> {
        struct V {}
        impl<'de> Visitor<'de> for V {
            type Value = ClientDownMessageList;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                todo!()
            }
        }
        d.deserialize_struct("ClientDownMessageList", &["commands"], V {})
    }
}
