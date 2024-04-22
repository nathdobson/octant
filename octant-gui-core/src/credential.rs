use serde::{Deserialize, Serialize};

use crate::public_key_credential::PublicKeyCredential;

#[derive(Serialize, Deserialize, Debug)]
pub enum Credential{
    PublicKeyCredential(PublicKeyCredential)
}