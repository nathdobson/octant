use serde::{Deserialize, Serialize};

use crate::TypeTag;

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct HtmlFormElementTag;

impl TypeTag for HtmlFormElementTag {}

#[derive(Serialize, Deserialize, Debug)]
pub enum HtmlFormElementMethod {
    SetListener,
    Enable,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HtmlFormElementUpMessage {
    Submit,
}
