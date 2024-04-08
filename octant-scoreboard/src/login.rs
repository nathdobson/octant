use std::sync::Arc;

use atomic_refcell::AtomicRefCell;
use url::Url;
use webauthn_rs::{prelude::Uuid, WebauthnBuilder};

use octant_gui::{
    builder::{ElementExt, HtmlFormElementExt},
    credential_creation_options,
    event_loop::Page,
    CredentialCreationOptions,
};
use octant_gui_core::{
    authenticator_attachment::AuthenticatorAttachment,
    authenticator_selection_criteria::AuthenticatorSelectionCriteria,
    pub_key_cred_params::PubKeyCredParams,
    public_key_credential_creation_options::PublicKeyCredentialCreationOptions,
    public_key_credential_rp_entity::PublicKeyCredentialRpEntity,
    public_key_credential_user_entity::PublicKeyCredentialUserEntity,
    user_verification_requirement::UserVerificationRequirement,
};
use octant_gui_core::attestation_conveyance_preference::AttestationConveyancePreference;
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

impl LoginHandler {
    pub fn do_register(session: Arc<Session>) {
        let rp_id = "localhost";
        let rp_origin = Url::parse("https://localhost:8080").expect("Invalid URL");
        let mut builder = WebauthnBuilder::new(rp_id, &rp_origin).expect("Invalid configuration");
        let webauthn = builder.build().expect("Invalid configuration");

        // Initiate a basic registration flow to enroll a cryptographic authenticator
        let (ccr, skr) = webauthn
            .start_passkey_registration(Uuid::new_v4(), "claire", "Claire", None)
            .expect("Failed to start registration.");
        let options = session.global().new_credential_creation_options();
        options.public_key(PublicKeyCredentialCreationOptions {
            challenge: ccr.public_key.challenge,
            rp: PublicKeyCredentialRpEntity {
                name: ccr.public_key.rp.name,
                icon: None,
                id: Some(ccr.public_key.rp.id),
            },
            user: PublicKeyCredentialUserEntity {
                name: ccr.public_key.user.name,
                display_name: ccr.public_key.user.display_name,
                id: ccr.public_key.user.id,
                icon: None,
            },
            pub_key_cred_params: ccr
                .public_key
                .pub_key_cred_params
                .into_iter()
                .map(|params| PubKeyCredParams {
                    typ: params.type_,
                    alg: params.alg,
                })
                .collect(),
            authenticator_selection: ccr.public_key.authenticator_selection.map(|selection| {
                AuthenticatorSelectionCriteria {
                    authenticator_attachment: selection.authenticator_attachment.map(
                        |attachment| match attachment {
                            webauthn_rs_core::proto::AuthenticatorAttachment::Platform => {
                                AuthenticatorAttachment::Platform
                            }
                            webauthn_rs_core::proto::AuthenticatorAttachment::CrossPlatform => {
                                AuthenticatorAttachment::CrossPlatform
                            }
                        },
                    ),
                    require_resident_key: selection.require_resident_key,
                    user_verification: match selection.user_verification {
                        webauthn_rs_core::proto::UserVerificationPolicy::Required => {
                            UserVerificationRequirement::Required
                        }
                        webauthn_rs_core::proto::UserVerificationPolicy::Preferred => {
                            UserVerificationRequirement::Preferred
                        }
                        webauthn_rs_core::proto::UserVerificationPolicy::Discouraged_DO_NOT_USE => {
                            UserVerificationRequirement::Discouraged
                        }
                    },
                }
            }),
            timeout: ccr.public_key.timeout,
            attestation: match ccr.public_key.attestation {
                None => AttestationConveyancePreference::None,
                Some(attestation) => match attestation {
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
            },
        });
        session
            .global()
            .window()
            .navigator()
            .credentials()
            .create_with_options(&options);
    }
}

impl Handler for LoginHandler {
    fn prefix(&self) -> String {
        "login".to_string()
    }

    fn handle(&self, url: &Url, session: Arc<Session>) -> anyhow::Result<Page> {
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
                    Self::do_register(session.clone());
                }
            });
        let page = d.create_element("div").child(text).child(form);
        Ok(Page::new(session.global().clone(), page))
    }
}
