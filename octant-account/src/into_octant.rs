use octant_gui_core::{AttestationConveyancePreference, AuthenticatorAttachment, AuthenticatorSelectionCriteria, PubKeyCredParams, PublicKeyCredentialCreationOptions, PublicKeyCredentialRpEntity, PublicKeyCredentialUserEntity, UserVerificationRequirement};

pub trait IntoOctant<O> {
    fn into_octant(self) -> O;
}

impl IntoOctant<PublicKeyCredentialCreationOptions>
for webauthn_rs_core::proto::PublicKeyCredentialCreationOptions
{
    fn into_octant(self) -> PublicKeyCredentialCreationOptions {
        PublicKeyCredentialCreationOptions {
            challenge: self.challenge,
            rp: self.rp.into_octant(),
            user: self.user.into_octant(),
            pub_key_cred_params: self.pub_key_cred_params.into_octant(),
            authenticator_selection: self.authenticator_selection.into_octant(),
            timeout: self.timeout,
            attestation: self.attestation.into_octant(),
        }
    }
}

impl IntoOctant<PublicKeyCredentialRpEntity> for webauthn_rs_core::proto::RelyingParty {
    fn into_octant(self) -> PublicKeyCredentialRpEntity {
        PublicKeyCredentialRpEntity {
            name: self.name,
            icon: None,
            id: Some(self.id),
        }
    }
}

impl IntoOctant<PublicKeyCredentialUserEntity> for webauthn_rs_core::proto::User {
    fn into_octant(self) -> PublicKeyCredentialUserEntity {
        PublicKeyCredentialUserEntity {
            name: self.name,
            display_name: self.display_name,
            id: self.id,
            icon: None,
        }
    }
}

impl IntoOctant<PubKeyCredParams> for webauthn_rs_core::proto::PubKeyCredParams {
    fn into_octant(self) -> PubKeyCredParams {
        PubKeyCredParams {
            typ: self.type_,
            alg: self.alg as i32,
        }
    }
}

impl IntoOctant<AttestationConveyancePreference>
for Option<webauthn_rs_core::proto::AttestationConveyancePreference>
{
    fn into_octant(self) -> AttestationConveyancePreference {
        match self {
            None => AttestationConveyancePreference::None,
            Some(this) => match this {
                webauthn_rs_core::proto::AttestationConveyancePreference::None => {
                    AttestationConveyancePreference::None
                }
                webauthn_rs_core::proto::AttestationConveyancePreference::Indirect => {
                    AttestationConveyancePreference::Indirect
                }
                webauthn_rs_core::proto::AttestationConveyancePreference::Direct => {
                    AttestationConveyancePreference::Direct
                }
            },
        }
    }
}

impl IntoOctant<AuthenticatorSelectionCriteria>
for webauthn_rs_core::proto::AuthenticatorSelectionCriteria
{
    fn into_octant(self) -> AuthenticatorSelectionCriteria {
        AuthenticatorSelectionCriteria {
            authenticator_attachment: self.authenticator_attachment.into_octant(),
            require_resident_key: self.require_resident_key,
            user_verification: self.user_verification.into_octant(),
        }
    }
}

impl IntoOctant<AuthenticatorAttachment> for webauthn_rs_core::proto::AuthenticatorAttachment {
    fn into_octant(self) -> AuthenticatorAttachment {
        match self {
            webauthn_rs_core::proto::AuthenticatorAttachment::Platform => {
                AuthenticatorAttachment::Platform
            }
            webauthn_rs_core::proto::AuthenticatorAttachment::CrossPlatform => {
                AuthenticatorAttachment::CrossPlatform
            }
        }
    }
}

impl IntoOctant<UserVerificationRequirement> for webauthn_rs_core::proto::UserVerificationPolicy {
    fn into_octant(self) -> UserVerificationRequirement {
        match self {
            webauthn_rs_core::proto::UserVerificationPolicy::Required => {
                UserVerificationRequirement::Required
            }
            webauthn_rs_core::proto::UserVerificationPolicy::Preferred => {
                UserVerificationRequirement::Preferred
            }
            webauthn_rs_core::proto::UserVerificationPolicy::Discouraged_DO_NOT_USE => {
                UserVerificationRequirement::Discouraged
            }
        }
    }
}

impl<A, B> IntoOctant<Vec<B>> for Vec<A>
    where
        A: IntoOctant<B>,
{
    fn into_octant(self) -> Vec<B> {
        self.into_iter().map(|x| x.into_octant()).collect()
    }
}

impl<A, B> IntoOctant<Option<B>> for Option<A>
    where
        A: IntoOctant<B>,
{
    fn into_octant(self) -> Option<B> {
        self.map(|x| x.into_octant())
    }
}