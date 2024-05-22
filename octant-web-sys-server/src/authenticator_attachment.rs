use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthenticatorAttachment {
    Platform,
    CrossPlatform,
}
