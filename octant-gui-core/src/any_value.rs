use serde::{Deserialize, Serialize};

use crate::TypeTag;

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct AnyValueTag;

impl TypeTag for AnyValueTag {}

#[derive(Serialize, Deserialize, Debug)]
pub enum AnyValueMethod {
    Downcast(JsClass),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum JsClass {
    Credential,
    Response,
}
