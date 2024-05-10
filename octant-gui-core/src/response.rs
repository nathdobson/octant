use serde::{Deserialize, Serialize};

use crate::TypeTag;

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct ResponseTag;

impl TypeTag for ResponseTag {}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseMethod {
    Text(),
}
