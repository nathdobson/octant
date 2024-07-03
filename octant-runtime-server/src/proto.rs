use std::{
    any::Any,
    fmt::{Debug, Formatter},
    rc::Rc,
};
use marshal_object::derive_box_object;
use octant_error::OctantResult;
use serde::{
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::runtime::Runtime;

#[cfg(side = "client")]
pub trait DownMessage: Debug + Any {
    fn run(self: Box<Self>, runtime: &Rc<Runtime>) -> OctantResult<()>;
}

#[cfg(side = "server")]
pub trait DownMessage: Debug + Any {}

#[cfg(side = "client")]
pub trait UpMessage: Debug + Any {}

#[cfg(side = "server")]
pub trait UpMessage: Debug + Any {
    fn run(self: Box<Self>, runtime: &Rc<Runtime>) -> OctantResult<()>;
}

struct BoxDownMessage;
derive_box_object!(BoxDownMessage, DownMessage);
struct BoxUpMessage;
derive_box_object!(BoxUpMessage, UpMessage);

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
