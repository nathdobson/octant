use base64urlsafedata::Base64UrlSafeData;
use serde::{Deserialize, Serialize};

use crate::{
    allow_credentials_type::AllowCredentialsType, authenticator_transport::AuthenticatorTransport,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct AllowCredentials {
    pub id: Base64UrlSafeData,
    pub transports: Vec<AuthenticatorTransport>,
    pub typ: AllowCredentialsType,
}
