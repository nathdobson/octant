use serde::{Deserialize, Serialize};

use crate::TypeTag;

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct WindowTag;

impl TypeTag for WindowTag {}

#[derive(Serialize, Deserialize, Debug)]
pub enum WindowMethod {
    Document,
}
