use base64urlsafedata::Base64UrlSafeData;
use marshal::{Deserialize, Serialize};

use crate::{
    authentication_extensions_client_outputs::AuthenticationExtensionsClientOutputs,
    authenticator_response::AuthenticatorResponse,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicKeyCredential {
    pub id: String,
    pub raw_id: Base64UrlSafeData,
    pub response: AuthenticatorResponse,
    pub extensions: AuthenticationExtensionsClientOutputs,
}
