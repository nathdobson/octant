use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum GlobalMethod {
    Window,
    NewCredentialCreationOptions,
    NewCredentialRequestOptions,
}