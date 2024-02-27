use serde::{Deserialize, Serialize};

use crate::TypeTag;

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct ElementTag;

impl TypeTag for ElementTag {}

#[derive(Serialize, Deserialize, Debug)]
pub enum ElementMethod {
    SetAttribute(String, String),
}
