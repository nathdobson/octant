use serde::{Deserialize, Serialize};

use crate::{TypedHandle, TypeTag};
use crate::node::NodeTag;

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct ElementTag;

impl TypeTag for ElementTag {}

#[derive(Serialize, Deserialize, Debug)]
pub enum ElementMethod {
    AppendChild(TypedHandle<NodeTag>),
    SetAttribute(String, String),
}
