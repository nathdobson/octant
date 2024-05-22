use crate::runtime::Runtime;
use octant_serde::{define_serde_trait, DeserializeWith, SerializeDyn, TypeMap};
use std::{any::Any, fmt::Debug, sync::Arc};
use std::fmt::Formatter;
use serde::{Deserializer, Serialize};
use serde::de::Visitor;

#[cfg(side="client")]
pub trait DownMessage: SerializeDyn + Debug + Any {
    fn run(self: Box<Self>, runtime: &Arc<Runtime>) -> anyhow::Result<()>;
}

#[cfg(side="server")]
pub trait DownMessage: SerializeDyn + Debug + Send + Sync + Any {
}

define_serde_trait!(DownMessage);

#[cfg(side="client")]
pub trait UpMessage: SerializeDyn + Debug + Any {
}

#[cfg(side="server")]
pub trait UpMessage: SerializeDyn + Debug + Send + Sync + Any {
    fn run(self: Box<Self>, runtime: &Arc<Runtime>) -> anyhow::Result<()>;
}

define_serde_trait!(UpMessage);

#[derive(Serialize, Debug)]
pub struct UpMessageList {
    pub commands: Vec<Box<dyn UpMessage>>,
}

#[derive(Serialize, Debug)]
pub struct DownMessageList {
    pub commands: Vec<Box<dyn DownMessage>>,
}

impl<'de> DeserializeWith<'de> for DownMessageList {
    fn deserialize_with<D: Deserializer<'de>>(ctx: &TypeMap, d: D) -> Result<Self, D::Error> {
        struct V {}
        impl<'de> Visitor<'de> for V {
            type Value = DownMessageList;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                todo!()
            }
        }
        d.deserialize_struct("DownMessageList", &["commands"], V {})
    }
}

impl<'de> DeserializeWith<'de> for UpMessageList {
    fn deserialize_with<D: Deserializer<'de>>(ctx: &TypeMap, d: D) -> Result<Self, D::Error> {
        struct V {}
        impl<'de> Visitor<'de> for V {
            type Value = UpMessageList;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                todo!()
            }
        }
        d.deserialize_struct("UpMessageList", &["commands"], V {})
    }
}
