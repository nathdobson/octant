use serde::{Deserialize, Serialize};

use crate::{TypeTag, TypedHandle};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct HtmlInputElementTag;

impl TypeTag for HtmlInputElementTag {}

#[derive(Serialize, Deserialize, Debug)]
pub enum HtmlInputElementUpMessage {
    SetInput { value: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HtmlInputElementMethod {}
