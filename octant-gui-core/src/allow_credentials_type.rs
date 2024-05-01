use crate::AuthenticatorTransport;
use base64urlsafedata::Base64UrlSafeData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum AllowCredentialsType {
    PublicKey,
}
