use serde::{Deserialize, Serialize};

use crate::{Error, TypeTag};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct PromiseTag;

impl TypeTag for PromiseTag {}

#[derive(Serialize, Deserialize, Debug)]
pub enum PromiseMethod {
    Wait,
    Get,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PromiseUpMessage {
    Done(Result<(), Error>),
}
