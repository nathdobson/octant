use octant_web_sys_server::{
    allow_credentials::AllowCredentials, allow_credentials_type::AllowCredentialsType,
    attestation_conveyance_preference::AttestationConveyancePreference,
    authentication_extensions_client_inputs::AuthenticationExtensionsClientInputs,
    authenticator_attachment::AuthenticatorAttachment,
    authenticator_selection_criteria::AuthenticatorSelectionCriteria,
    authenticator_transport::AuthenticatorTransport, pub_key_cred_params::PubKeyCredParams,
    public_key_credential_creation_options::PublicKeyCredentialCreationOptions,
    public_key_credential_request_options::PublicKeyCredentialRequestOptions,
    public_key_credential_rp_entity::PublicKeyCredentialRpEntity,
    public_key_credential_user_entity::PublicKeyCredentialUserEntity,
    user_verification_requirement::UserVerificationRequirement,
};

pub trait IntoOctant<O> {
    fn into_octant(self) -> O;
}

impl IntoOctant<PublicKeyCredentialCreationOptions>
    for webauthn_rs_proto::PublicKeyCredentialCreationOptions
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
            extensions: self.extensions.into_octant(),
        }
    }
}

impl IntoOctant<PublicKeyCredentialRpEntity> for webauthn_rs_proto::RelyingParty {
    fn into_octant(self) -> PublicKeyCredentialRpEntity {
        PublicKeyCredentialRpEntity {
            name: self.name,
            icon: None,
            id: Some(self.id),
        }
    }
}

impl IntoOctant<PublicKeyCredentialUserEntity> for webauthn_rs_proto::User {
    fn into_octant(self) -> PublicKeyCredentialUserEntity {
        PublicKeyCredentialUserEntity {
            name: self.name,
            display_name: self.display_name,
            id: self.id,
            icon: None,
        }
    }
}

impl IntoOctant<PubKeyCredParams> for webauthn_rs_proto::PubKeyCredParams {
    fn into_octant(self) -> PubKeyCredParams {
        PubKeyCredParams {
            typ: self.type_,
            alg: self.alg as i32,
        }
    }
}

impl IntoOctant<AttestationConveyancePreference>
    for Option<webauthn_rs_proto::AttestationConveyancePreference>
{
    fn into_octant(self) -> AttestationConveyancePreference {
        match self {
            None => AttestationConveyancePreference::None,
            Some(this) => match this {
                webauthn_rs_proto::AttestationConveyancePreference::None => {
                    AttestationConveyancePreference::None
                }
                webauthn_rs_proto::AttestationConveyancePreference::Indirect => {
                    AttestationConveyancePreference::Indirect
                }
                webauthn_rs_proto::AttestationConveyancePreference::Direct => {
                    AttestationConveyancePreference::Direct
                }
            },
        }
    }
}

impl IntoOctant<AuthenticatorSelectionCriteria>
    for webauthn_rs_proto::AuthenticatorSelectionCriteria
{
    fn into_octant(self) -> AuthenticatorSelectionCriteria {
        AuthenticatorSelectionCriteria {
            authenticator_attachment: self.authenticator_attachment.into_octant(),
            require_resident_key: self.require_resident_key,
            user_verification: self.user_verification.into_octant(),
        }
    }
}

impl IntoOctant<AuthenticatorAttachment> for webauthn_rs_proto::AuthenticatorAttachment {
    fn into_octant(self) -> AuthenticatorAttachment {
        match self {
            webauthn_rs_proto::AuthenticatorAttachment::Platform => {
                AuthenticatorAttachment::Platform
            }
            webauthn_rs_proto::AuthenticatorAttachment::CrossPlatform => {
                AuthenticatorAttachment::CrossPlatform
            }
        }
    }
}

impl IntoOctant<UserVerificationRequirement> for webauthn_rs_proto::UserVerificationPolicy {
    fn into_octant(self) -> UserVerificationRequirement {
        match self {
            webauthn_rs_proto::UserVerificationPolicy::Required => {
                UserVerificationRequirement::Required
            }
            webauthn_rs_proto::UserVerificationPolicy::Preferred => {
                UserVerificationRequirement::Preferred
            }
            webauthn_rs_proto::UserVerificationPolicy::Discouraged_DO_NOT_USE => {
                UserVerificationRequirement::Discouraged
            }
        }
    }
}

impl IntoOctant<AuthenticationExtensionsClientInputs>
    for webauthn_rs_proto::RequestRegistrationExtensions
{
    fn into_octant(self) -> AuthenticationExtensionsClientInputs {
        AuthenticationExtensionsClientInputs {}
    }
}

impl IntoOctant<PublicKeyCredentialRequestOptions>
    for webauthn_rs_proto::PublicKeyCredentialRequestOptions
{
    fn into_octant(self) -> PublicKeyCredentialRequestOptions {
        PublicKeyCredentialRequestOptions {
            challenge: self.challenge,
            timeout: self.timeout,
            rp_id: self.rp_id,
            allow_credentials: self.allow_credentials.into_octant(),
            user_verification: self.user_verification.into_octant(),
            extensions: self.extensions.into_octant(),
        }
    }
}

impl IntoOctant<AllowCredentials> for webauthn_rs_proto::AllowCredentials {
    fn into_octant(self) -> AllowCredentials {
        AllowCredentials {
            id: self.id,
            transports: self.transports.unwrap_or_default().into_octant(),
            typ: match &*self.type_ {
                "public-key" => AllowCredentialsType::PublicKey,
                _ => todo!(),
            },
        }
    }
}

impl IntoOctant<AuthenticatorTransport> for webauthn_rs_proto::AuthenticatorTransport {
    fn into_octant(self) -> AuthenticatorTransport {
        match self {
            webauthn_rs_proto::AuthenticatorTransport::Usb => AuthenticatorTransport::Usb,
            webauthn_rs_proto::AuthenticatorTransport::Nfc => AuthenticatorTransport::Nfc,
            webauthn_rs_proto::AuthenticatorTransport::Ble => AuthenticatorTransport::Ble,
            webauthn_rs_proto::AuthenticatorTransport::Internal => AuthenticatorTransport::Internal,
            webauthn_rs_proto::AuthenticatorTransport::Hybrid => AuthenticatorTransport::Hybrid,
            webauthn_rs_proto::AuthenticatorTransport::Test => AuthenticatorTransport::Test,
            webauthn_rs_proto::AuthenticatorTransport::Unknown => AuthenticatorTransport::Unknown,
        }
    }
}

impl IntoOctant<AuthenticationExtensionsClientInputs>
    for webauthn_rs_proto::RequestAuthenticationExtensions
{
    fn into_octant(self) -> AuthenticationExtensionsClientInputs {
        todo!()
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
