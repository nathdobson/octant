use serde::{Deserialize, Serialize};

use crate::TypeTag;

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct RequestInitTag;

impl TypeTag for RequestInitTag {}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestInitMethod {}
