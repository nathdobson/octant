use crate::{
    authenticator_attachment::AuthenticatorAttachment,
    user_verification_requirement::UserVerificationRequirement,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticatorSelectionCriteria {
    pub authenticator_attachment: Option<AuthenticatorAttachment>,
    pub require_resident_key: bool,
    pub user_verification: UserVerificationRequirement,
}
