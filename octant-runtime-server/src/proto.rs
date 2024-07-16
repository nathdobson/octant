use crate::runtime::Runtime;
use marshal::{context::Context, Deserialize, Serialize};
use marshal_fixed::{
    decode::full::{FixedDecoder, FixedDecoderBuilder},
    encode::full::{FixedEncoder, FixedEncoderBuilder},
    DeserializeFixed, SerializeFixed,
};
use marshal_json::{
    decode::full::{JsonDecoder, JsonDecoderBuilder},
    encode::full::{JsonEncoder, JsonEncoderBuilder},
    DeserializeJson, SerializeJson,
};
use marshal_object::{
    derive_box_object, derive_deserialize_provider, derive_serialize_provider, AsDiscriminant,
};
use marshal_pointer::raw_any::RawAny;
use octant_error::{octant_error, OctantError, OctantResult};
use std::{
    fmt::{Debug, Display, Formatter},
    rc::Rc,
    str::FromStr,
};
use anyhow::Context as _;

#[cfg(side = "client")]
pub trait DownMessage: Debug + RawAny + AsDiscriminant<BoxDownMessage> {
    fn run(self: Box<Self>, runtime: &Rc<Runtime>) -> OctantResult<()>;
}

#[cfg(side = "server")]
pub trait DownMessage: Debug + RawAny + AsDiscriminant<BoxDownMessage> {}

#[cfg(side = "client")]
pub trait UpMessage: Debug + RawAny + AsDiscriminant<BoxUpMessage> {}

#[cfg(side = "server")]
pub trait UpMessage: Debug + RawAny + AsDiscriminant<BoxUpMessage> {
    fn run(self: Box<Self>, runtime: &Rc<Runtime>) -> OctantResult<()>;
}

pub struct BoxDownMessage;
derive_box_object!(BoxDownMessage, DownMessage);
derive_serialize_provider!(BoxDownMessage, JsonEncoder, FixedEncoder);
derive_deserialize_provider!(BoxDownMessage, JsonDecoder, FixedDecoder);

pub struct BoxUpMessage;
derive_box_object!(BoxUpMessage, UpMessage);
derive_serialize_provider!(BoxUpMessage, JsonEncoder, FixedEncoder);
derive_deserialize_provider!(BoxUpMessage, JsonDecoder, FixedDecoder);

#[derive(Serialize, Deserialize, Debug)]
pub struct UpMessageList {
    pub commands: Vec<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownMessageList {
    pub commands: Vec<Vec<u8>>,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Copy, Clone)]
pub enum Proto {
    Json,
    Fixed,
}

impl FromStr for Proto {
    type Err = OctantError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Proto::Json),
            "fixed" => Ok(Proto::Fixed),
            _ => Err(octant_error!("unexpected proto")),
        }
    }
}

impl Proto {
    pub fn deserialize<T: DeserializeJson + DeserializeFixed>(
        &self,
        data: &[u8],
        ctx: Context,
    ) -> anyhow::Result<T> {
        match self {
            Proto::Json => Ok(JsonDecoderBuilder::new(data).deserialize(ctx).context("when deserializing json")?),
            Proto::Fixed => Ok(FixedDecoderBuilder::new(data).deserialize(ctx).context("when deserializing fixed")?),
        }
    }
    pub fn serialize<T: SerializeJson + SerializeFixed>(
        &self,
        value: &T,
        ctx: Context,
    ) -> anyhow::Result<Vec<u8>> {
        match self {
            Proto::Json => Ok(JsonEncoderBuilder::new()
                .serialize(value, ctx)?
                .into_bytes()),
            Proto::Fixed => Ok(FixedEncoderBuilder::new().serialize(value, ctx)?),
        }
    }
}

impl Display for Proto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Proto::Json => write!(f, "json"),
            Proto::Fixed => write!(f, "fixed"),
        }
    }
}
