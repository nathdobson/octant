use serde::{Deserialize, Serialize};

use crate::{TypedHandle, TypeTag};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct NodeTag;

impl TypeTag for NodeTag {}

#[derive(Serialize, Deserialize, Debug)]
pub enum NodeMethod {
    AppendChild(TypedHandle<NodeTag>),
    RemoveChild(TypedHandle<NodeTag>),
    SetNodeValue(Option<String>),
}
