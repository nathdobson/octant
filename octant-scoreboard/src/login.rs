use std::sync::Arc;

use anyhow::anyhow;
use atomic_refcell::AtomicRefCell;
use url::Url;
use webauthn_rs::{prelude::Uuid, WebauthnBuilder};
use webauthn_rs_core::proto::{AuthenticatorAttestationResponseRaw, RegisterPublicKeyCredential};

use octant_gui::{
    builder::{ElementExt, HtmlFormElementExt},
    event_loop::Page,
};
use octant_gui_core::{
    AttestationConveyancePreference, AuthenticatorAttachment, AuthenticatorAttestationResponse,
    AuthenticatorResponse, AuthenticatorSelectionCriteria, Credential, PubKeyCredParams,
    PublicKeyCredential, PublicKeyCredentialCreationOptions, PublicKeyCredentialRpEntity,
    PublicKeyCredentialUserEntity, RegistrationExtensionsClientOutputs,
    UserVerificationRequirement,
};
use octant_server::{
    session::{Session, SessionData},
    Handler,
};

pub struct LoginHandler {}

struct UserId(u64);

#[derive(Default)]
struct LoginState {
    verified_user: Option<UserId>,
}

#[derive(Default)]
struct LoginSession {
    state: AtomicRefCell<LoginState>,
}

impl SessionData for LoginSession {}

trait IntoOctant<O> {
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

trait IntoAuth<O> {
    fn into_auth(self) -> O;
}

impl IntoAuth<RegisterPublicKeyCredential> for Credential {
    fn into_auth(self) -> RegisterPublicKeyCredential {
        match self {
            Credential::PublicKeyCredential(credential) => credential.into_auth(),
        }
    }
}

impl IntoAuth<RegisterPublicKeyCredential> for PublicKeyCredential {
    fn into_auth(self) -> RegisterPublicKeyCredential {
        RegisterPublicKeyCredential {
            id: self.id,
            raw_id: self.raw_id,
            response: self.response.into_auth(),
            type_: "PublicKeyCredential".to_string(),
            extensions: self.extensions.into_auth(),
        }
    }
}

impl IntoAuth<webauthn_rs_core::proto::AuthenticatorAttestationResponseRaw>
    for AuthenticatorResponse
{
    fn into_auth(self) -> webauthn_rs_core::proto::AuthenticatorAttestationResponseRaw {
        match self {
            AuthenticatorResponse::AuthenticatorAttestationResponse(resp) => resp.into_auth(),
        }
    }
}

impl IntoAuth<webauthn_rs_core::proto::RegistrationExtensionsClientOutputs>
    for RegistrationExtensionsClientOutputs
{
    fn into_auth(self) -> webauthn_rs_core::proto::RegistrationExtensionsClientOutputs {
        webauthn_rs_core::proto::RegistrationExtensionsClientOutputs {
            appid: None,
            cred_props: None,
            hmac_secret: None,
            cred_protect: None,
            min_pin_length: None,
        }
    }
}

impl IntoAuth<webauthn_rs_core::proto::AuthenticatorAttestationResponseRaw>
    for AuthenticatorAttestationResponse
{
    fn into_auth(self) -> AuthenticatorAttestationResponseRaw {
        AuthenticatorAttestationResponseRaw {
            attestation_object: self.attestation_object,
            client_data_json: self.client_data_json,
            transports: None,
        }
    }
}

impl LoginHandler {
    pub fn do_register(session: Arc<Session>, url: &Url) -> anyhow::Result<()> {
        let host = url
            .host()
            .ok_or_else(|| anyhow!("host not included in URL"))?;
        let rp_id = format!("{}", host);
        let rp_origin = url.join("/").expect("Invalid URL");
        let builder = WebauthnBuilder::new(&rp_id, &rp_origin).expect("Invalid configuration");
        let webauthn = builder.build().expect("Invalid configuration");
        let (ccr, skr) = webauthn
            .start_passkey_registration(Uuid::new_v4(), "claire", "Claire", None)
            .expect("Failed to start registration.");
        let options = session.global().new_credential_creation_options();
        options.public_key(ccr.public_key.into_octant());
        let p = session
            .global()
            .window()
            .navigator()
            .credentials()
            .create_with_options(&options);
        p.wait();
        tokio::spawn(async move {
            let cred = match p.get().await {
                Err(e) => {
                    log::error!("{:?}", e);
                    return;
                }
                Ok(x) => x,
            };
            let cred = cred.into_auth();
            let result = webauthn.finish_passkey_registration(&cred, &skr).unwrap();
            log::info!("{:?}", result);
        });
        Ok(())
    }
}

impl Handler for LoginHandler {
    fn prefix(&self) -> String {
        "login".to_string()
    }

    fn handle(&self, url: &Url, session: Arc<Session>) -> anyhow::Result<Page> {
        let url = url.clone();
        let d = session.global().window().document();
        let text = d.create_text_node("Login");
        let input = d
            .create_input_element()
            .attr("type", "text")
            .attr("placeholder", "Team Name");
        let form = d
            .create_form_element()
            .child(input.clone())
            .child(d.create_element("br"))
            .child(
                d.create_input_element()
                    .attr("type", "submit")
                    .attr("value", "Login"),
            )
            .handler({
                let session = session.clone();
                let text = text.clone();
                move || {
                    // let data = session.data::<LoginSession>();
                    // let ref mut state = *data.state.borrow_mut();
                    if let Err(e) = Self::do_register(session.clone(), &url) {
                        log::error!("{:?}", e);
                    }
                }
            });
        let page = d.create_element("div").child(text).child(form);
        Ok(Page::new(session.global().clone(), page))
    }
}
