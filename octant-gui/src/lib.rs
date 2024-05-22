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
    fmt::{Debug, Formatter},
    pin::Pin,
    sync::Arc,
};

use catalog::{Builder, BuilderFrom, Registry};
use futures::{Sink, Stream};
use serde::{de::Visitor, Deserializer, Serialize};
use type_map::TypeMap;

use octant_gui_core::reexports::octant_serde::{define_serde_trait, DeserializeWith, SerializeDyn};
pub use runtime::Runtime;

pub mod event_loop;
pub mod handle;
pub mod runtime;
pub mod sink;

//
pub type UpMessageStream =
    Pin<Box<dyn Send + Sync + Stream<Item = anyhow::Result<Option<ServerUpMessageList>>>>>;

type DynUpMessageHandler = Box<
    dyn 'static
        + Send
        + Sync
        + for<'a> Fn(&'a Arc<Runtime>, Box<dyn ServerUpMessage>) -> anyhow::Result<()>,
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

impl<T: ServerUpMessage> BuilderFrom<UpMessageHandler<T>> for UpMessageHandlerRegistry {
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

pub trait ServerUpMessage: SerializeDyn + Debug + Send + Sync + Any {}
define_serde_trait!(ServerUpMessage);

#[derive(Serialize, Debug)]
pub struct ServerDownMessageList {
    pub commands: Vec<Box<dyn ServerDownMessage>>,
}

pub struct ServerUpMessageList {
    pub commands: Vec<Box<dyn ServerUpMessage>>,
}

impl<'de> DeserializeWith<'de> for ServerUpMessageList {
    fn deserialize_with<D: Deserializer<'de>>(ctx: &TypeMap, d: D) -> Result<Self, D::Error> {
        struct V {}
        impl<'de> Visitor<'de> for V {
            type Value = ServerUpMessageList;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                todo!()
            }
        }
        d.deserialize_struct("ServerUpMessageList", &["commands"], V {})
    }
}
