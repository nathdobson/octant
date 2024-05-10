use serde::{Deserialize, Serialize};

use crate::TypeTag;

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct RequestTag;

impl TypeTag for RequestTag {}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestMethod {}
