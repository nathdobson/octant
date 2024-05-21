#![allow(dead_code)]
#![deny(unused_must_use)]
#![feature(try_blocks)]
#![feature(trait_alias)]
#![feature(trait_upcasting)]
#![feature(ptr_metadata)]
#![allow(unused_variables)]

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    pin::Pin,
    sync::Arc,
};

use catalog::{Builder, BuilderFrom, Registry};
use futures::{Sink, Stream};
use serde::Serialize;

pub use global::Global;
use octant_gui_core::{
    NewUpMessage,
    reexports::octant_serde::{define_serde_trait, SerializeDyn}, UpMessageList,
};
pub use runtime::Runtime;

pub mod builder;
pub mod document;
pub mod element;
pub mod event_loop;
pub mod global;
pub mod handle;
pub mod html_element;
pub mod html_form_element;
pub mod html_input_element;
pub mod js_value;
pub mod node;
pub mod object;
pub mod runtime;
pub mod text;
pub mod window;

mod any_value;
mod credential;
pub mod credential_creation_options;
mod credential_request_options;
pub mod credentials_container;
pub mod navigator;
mod promise;
mod request;
mod request_init;
mod response;
pub mod sink;
//
pub type UpMessageStream =
    Pin<Box<dyn Send + Sync + Stream<Item = anyhow::Result<Option<UpMessageList>>>>>;
// pub type Handle = Arc<dyn handle::Trait>;
// pub type JsValue = Arc<dyn js_value::Trait>;
// pub type Window = Arc<dyn window::Trait>;
//
// pub type Document = Arc<dyn document::Trait>;
//
// pub type Navigator = Arc<dyn navigator::Trait>;
//
// pub type CredentialsContainer = Arc<dyn credentials_container::Trait>;
// pub type CredentialCreationOptions = Arc<dyn credential_creation_options::Trait>;
// pub type CredentialRequestOptions = Arc<dyn credential_request_options::Trait>;
// pub type RequestInit = Arc<dyn request_init::Trait>;
// pub type Request = Arc<dyn request::Trait>;
//
// pub type HtmlElement = Arc<dyn html_element::Trait>;
// pub type HtmlFormElement = Arc<dyn html_form_element::Trait>;
// pub type HtmlInputElement = Arc<dyn html_input_element::Trait>;
//
// pub type Element = Arc<dyn element::Trait>;
//
// pub type Node = Arc<dyn node::Trait>;
//
// pub type Text = Arc<dyn text::Trait>;
// pub type AnyValue = Arc<dyn any_value::Trait>;
//
// pub type Promise = Arc<dyn promise::Trait>;
//
// pub type Credential = Arc<dyn credential::Trait>;
// pub type Response = Arc<dyn response::Trait>;

type DynUpMessageHandler = Box<
    dyn 'static
        + Send
        + Sync
        + for<'a> Fn(&'a Arc<Runtime>, Box<dyn NewUpMessage>) -> anyhow::Result<()>,
>;

pub struct UpMessageHandlerRegistry {
    handlers: HashMap<TypeId, DynUpMessageHandler>,
}

impl Builder for UpMessageHandlerRegistry {
    type Output = Self;
    fn new() -> Self {
        UpMessageHandlerRegistry {
            handlers: HashMap::new(),
        }
    }
    fn build(self) -> Self::Output {
        self
    }
}

impl<T: NewUpMessage> BuilderFrom<UpMessageHandler<T>> for UpMessageHandlerRegistry {
    fn insert(&mut self, handler: UpMessageHandler<T>) {
        self.handlers.insert(
            TypeId::of::<T>(),
            Box::new(move |ctx, message| {
                let message = Box::<dyn Any>::downcast(message as Box<dyn Any>);
                let message = *message.ok().unwrap();
                handler(ctx, message)
            }),
        );
    }
}

pub static UP_MESSAGE_HANDLER_REGISTRY: Registry<UpMessageHandlerRegistry> = Registry::new();

pub type UpMessageHandler<T> = for<'a> fn(&'a Arc<Runtime>, T) -> anyhow::Result<()>;

pub type DownMessageSink =
    Pin<Box<dyn Send + Sync + Sink<ServerDownMessageList, Error = anyhow::Error>>>;

pub trait ServerDownMessage: SerializeDyn + Debug + Send + Sync + Any {}
define_serde_trait!(ServerDownMessage);

#[derive(Serialize, Debug)]
pub struct ServerDownMessageList {
    pub commands: Vec<Box<dyn ServerDownMessage>>,
}
