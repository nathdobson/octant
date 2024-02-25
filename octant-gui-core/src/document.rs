use serde::{Deserialize, Serialize};

use crate::TypeTag;

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct DocumentTag;

impl TypeTag for DocumentTag {}

#[derive(Serialize, Deserialize, Debug)]
pub enum DocumentMethod {
    Body,
    CreateTextNode(String),
    CreateElement(String),
    CreateFormElement,
    CreateInputElement,
}
