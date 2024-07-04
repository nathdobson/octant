use base64urlsafedata::Base64UrlSafeData;
use marshal::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticatorAssertionResponse {
    pub authenticator_data: Base64UrlSafeData,
    pub client_data_json: Base64UrlSafeData,
    pub signature: Base64UrlSafeData,
    pub user_handle: Option<Base64UrlSafeData>,
}
