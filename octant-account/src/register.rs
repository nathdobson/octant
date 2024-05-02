use std::sync::Arc;

use anyhow::{ Context};
use url::Url;
use webauthn_rs::{prelude::Uuid};

use octant_gui::{
    builder::{ElementExt, HtmlFormElementExt},
    event_loop::Page,
};
use octant_server::{Handler, session::Session};

use crate::{build_webauthn, into_auth::IntoAuth, into_octant::IntoOctant};

pub struct RegisterHandler {}

impl RegisterHandler {
    pub fn do_register(
        session: Arc<Session>,
        url: &Url,
        email: &str,
        name: &str,
    ) -> anyhow::Result<()> {
        let webauthn = build_webauthn(url)?;
        let (ccr, skr) = webauthn.start_passkey_registration(Uuid::new_v4(), email, name, None)?;
        let options = session.global().new_credential_creation_options();
        options.public_key(ccr.public_key.into_octant());
        let p = session
            .global()
            .window()
            .navigator()
            .credentials()
            .create_with_options(&options);
        tokio::spawn(async move {
            let cred = match p.get().await {
                Err(e) => {
                    log::error!("{:?}", e);
                    return;
                }
                Ok(x) => x,
            };
            let cred = cred.into_auth();
            let result = webauthn
                .finish_passkey_registration(&cred, &skr)
                .context("while verifying passkey")
                .unwrap();
            log::info!("{:?}", result);
        });
        Ok(())
    }
}

impl Handler for RegisterHandler {
    fn prefix(&self) -> String {
        "register".to_string()
    }

    fn handle(&self, url: &Url, session: Arc<Session>) -> anyhow::Result<Page> {
        let url = url.clone();
        let d = session.global().window().document();
        let text = d.create_text_node("Register");
        let email = d
            .create_input_element()
            .attr("type", "text")
            .attr("placeholder", "Email")
            .attr("required", "true");
        let name = d
            .create_input_element()
            .attr("type", "text")
            .attr("placeholder", "Team Name")
            .attr("required", "true");
        let form = d
            .create_form_element()
            .child(email.clone())
            .child(d.create_element("br"))
            .child(name.clone())
            .child(d.create_element("br"))
            .child(
                d.create_input_element()
                    .attr("type", "submit")
                    .attr("value", "Register"),
            )
            .handler({
                let session = session.clone();
                let email = email.clone();
                let name = name.clone();
                move || {
                    if let Err(e) = Self::do_register(
                        session.clone(),
                        &url,
                        &email.input_value(),
                        &name.input_value(),
                    ) {
                        session.global().fail(e);
                    }
                }
            });
        let page = d.create_element("div").child(text).child(form);
        Ok(Page::new(session.global().clone(), page))
    }
}
