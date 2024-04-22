use serde::{Deserialize, Serialize};

use crate::TypeTag;

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct PromiseTag;

impl TypeTag for PromiseTag {}

#[derive(Serialize, Deserialize, Debug)]
pub enum PromiseMethod {}

#[derive(Serialize, Deserialize, Debug)]
pub enum PromiseUpMessage {}
