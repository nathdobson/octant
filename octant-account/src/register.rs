use std::sync::Arc;

use anyhow::anyhow;
use url::Url;
use webauthn_rs::{prelude::Uuid, WebauthnBuilder};

use octant_gui::{
    builder::{ElementExt, HtmlFormElementExt},
    event_loop::Page,
};
use octant_server::{session::Session, Handler};

use crate::{into_auth::IntoAuth, into_octant::IntoOctant};

pub struct RegisterHandler {}

impl RegisterHandler {
    pub fn do_register(session: Arc<Session>, url: &Url, name: &str) -> anyhow::Result<()> {
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

impl Handler for RegisterHandler {
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
            .attr("placeholder", "Team Name")
            .attr("required", "true");
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
                let input = input.clone();
                move || {
                    // let data = session.data::<LoginSession>();
                    // let ref mut state = *data.state.borrow_mut();
                    if let Err(e) = Self::do_register(session.clone(), &url, &input.input_value()) {
                        log::error!("{:?}", e);
                    }
                }
            });
        let page = d.create_element("div").child(text).child(form);
        Ok(Page::new(session.global().clone(), page))
    }
}
