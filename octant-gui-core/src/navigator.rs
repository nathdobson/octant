use serde::{Deserialize, Serialize};

use crate::TypeTag;

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct NavigatorTag;

impl TypeTag for NavigatorTag {}

#[derive(Serialize, Deserialize, Debug)]
pub enum NavigatorMethod {
    Credentials
}
