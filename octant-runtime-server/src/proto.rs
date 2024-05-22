use crate::runtime::Runtime;
use octant_serde::{
    define_serde_trait, DeserializeWith, DeserializeWithSeed, SerializeDyn, TypeMap,
};
use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::{
    any::Any,
    fmt::{Debug, Formatter},
    sync::Arc,
};
use serde::de::Error;

#[cfg(side = "client")]
pub trait DownMessage: SerializeDyn + Debug + Any {
    fn run(self: Box<Self>, runtime: &Arc<Runtime>) -> anyhow::Result<()>;
}

#[cfg(side = "server")]
pub trait DownMessage: SerializeDyn + Debug + Send + Sync + Any {}

define_serde_trait!(DownMessage);

#[cfg(side = "client")]
pub trait UpMessage: SerializeDyn + Debug + Any {}

#[cfg(side = "server")]
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
        struct V<'c> {
            ctx: &'c TypeMap,
        }
        impl<'c, 'de> Visitor<'de> for V<'c> {
            type Value = DownMessageList;
            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "DownMessageList")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                #[derive(Deserialize)]
                enum Field {
                    #[serde(rename = "commands")]
                    Commands,
                }
                let commands = match map
                    .next_key::<Field>()?
                    .ok_or_else(|| A::Error::custom("missing commands"))?
                {
                    Field::Commands => map.next_value_seed(DeserializeWithSeed::<
                        Vec<Box<dyn DownMessage>>,
                    >::new(self.ctx))?,
                };
                Ok(DownMessageList { commands })
            }
        }
        d.deserialize_struct("DownMessageList", &["commands"], V { ctx })
    }
}

impl<'de> DeserializeWith<'de> for UpMessageList {
    fn deserialize_with<D: Deserializer<'de>>(ctx: &TypeMap, d: D) -> Result<Self, D::Error> {
        struct V {}
        impl<'de> Visitor<'de> for V {
            type Value = UpMessageList;
            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "DownMessageList")
            }
        }
        d.deserialize_struct("UpMessageList", &["commands"], V {})
    }
}
