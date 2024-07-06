use std::{
    fmt::Debug,
    rc::Rc,
};
use marshal::{Deserialize, Serialize};
use marshal_bin::{decode::full::BinDecoder, encode::full::BinEncoder};
use marshal_json::{decode::full::JsonDecoder, encode::full::JsonEncoder};
use marshal_object::{
    AsDiscriminant, derive_box_object, derive_deserialize_provider, derive_serialize_provider,
};
use marshal_pointer::raw_any::RawAny;
use octant_error::OctantResult;

use crate::runtime::Runtime;

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
derive_serialize_provider!(BoxDownMessage, JsonEncoder, BinEncoder);
derive_deserialize_provider!(BoxDownMessage, JsonDecoder, BinDecoder);

pub struct BoxUpMessage;
derive_box_object!(BoxUpMessage, UpMessage);
derive_serialize_provider!(BoxUpMessage, JsonEncoder, BinEncoder);
derive_deserialize_provider!(BoxUpMessage, JsonDecoder, BinDecoder);

#[derive(Serialize, Deserialize, Debug)]
pub struct UpMessageList {
    pub commands: Vec<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownMessageList {
    pub commands: Vec<Vec<u8>>,
}

// impl<'de> DeserializeWith<'de> for DownMessageList {
//     fn deserialize_with<D: Deserializer<'de>>(
//         ctx: &DeserializeContext,
//         d: D,
//     ) -> Result<Self, D::Error> {
//         struct V<'c> {
//             ctx: &'c DeserializeContext,
//         }
//         impl<'c, 'de> Visitor<'de> for V<'c> {
//             type Value = DownMessageList;
//             fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
//                 write!(f, "DownMessageList")
//             }
//             fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
//             where
//                 A: MapAccess<'de>,
//             {
//                 #[derive(Deserialize)]
//                 enum Field {
//                     #[serde(rename = "commands")]
//                     Commands,
//                 }
//                 let commands = match map
//                     .next_key::<Field>()?
//                     .ok_or_else(|| A::Error::custom("missing commands"))?
//                 {
//                     Field::Commands => map.next_value_seed(DeserializeWithSeed::<
//                         Vec<Encoded<dyn DownMessage>>,
//                     >::new(self.ctx))?,
//                 };
//                 Ok(DownMessageList { commands })
//             }
//         }
//         d.deserialize_struct("DownMessageList", &["commands"], V { ctx })
//     }
// }

// impl<'de> DeserializeWith<'de> for UpMessageList {
//     fn deserialize_with<D: Deserializer<'de>>(
//         ctx: &DeserializeContext,
//         d: D,
//     ) -> Result<Self, D::Error> {
//         struct V {}
//         impl<'de> Visitor<'de> for V {
//             type Value = UpMessageList;
//             fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
//                 write!(f, "DownMessageList")
//             }
//         }
//         d.deserialize_struct("UpMessageList", &["commands"], V {})
//     }
// }
