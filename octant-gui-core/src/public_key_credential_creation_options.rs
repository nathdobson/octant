use base64urlsafedata::Base64UrlSafeData;
use serde::{Deserialize, Serialize};

use crate::{
    attestation_conveyance_preference::AttestationConveyancePreference,
    authenticator_selection_criteria::AuthenticatorSelectionCriteria,
    pub_key_cred_params::PubKeyCredParams,
    public_key_credential_rp_entity::PublicKeyCredentialRpEntity,
    public_key_credential_user_entity::PublicKeyCredentialUserEntity,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicKeyCredentialCreationOptions {
    pub challenge: Base64UrlSafeData,
    pub rp: PublicKeyCredentialRpEntity,
    pub user: PublicKeyCredentialUserEntity,
    pub pub_key_cred_params: Vec<PubKeyCredParams>,
    pub authenticator_selection: Option<AuthenticatorSelectionCriteria>,
    pub timeout: Option<u32>,
    pub attestation: AttestationConveyancePreference,
}
