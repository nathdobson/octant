use serde::{Deserialize, Serialize};

use crate::{RequestInitTag, TypedHandle};

#[derive(Serialize, Deserialize, Debug)]
pub enum GlobalMethod {
    Window,
    NewCredentialCreationOptions,
    NewCredentialRequestOptions,
    NewRequestInit,
    NewRequest(String, TypedHandle<RequestInitTag>),
}
