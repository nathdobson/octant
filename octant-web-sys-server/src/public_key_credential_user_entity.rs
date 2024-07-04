use base64urlsafedata::Base64UrlSafeData;
use marshal::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicKeyCredentialUserEntity {
    pub name: String,
    pub display_name: String,
    pub id: Base64UrlSafeData,
    pub icon: Option<String>,
}
