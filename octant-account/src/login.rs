use std::sync::Arc;

use url::Url;

use octant_database::tree::Tree;
use octant_gui::{
    builder::{ElementExt, HtmlFormElementExt},
    event_loop::Page,
};
use octant_server::{Handler, session::Session};

use crate::{AccountDatabase, build_webauthn, into_auth::IntoAuth, into_octant::IntoOctant};

pub struct LoginHandler {
    pub database: Arc<Tree<AccountDatabase>>,
}

impl LoginHandler {
    pub fn do_login(session: Arc<Session>, url: &Url, email: &str) -> anyhow::Result<()> {
        let webauthn = build_webauthn(url)?;
        let (rcr, skr) = webauthn.start_passkey_authentication(&[])?;
        let options = session.global().new_credential_request_options();
        options.public_key(rcr.public_key.into_octant());
        let p = session
            .global()
            .window()
            .navigator()
            .credentials()
            .get_with_options(&options);
        tokio::spawn(async move {
            let cred = match p.get().await {
                Err(e) => {
                    log::error!("{:?}", e);
                    return;
                }
                Ok(x) => x,
            };
            let cred = cred.into_auth();
            let result = webauthn.finish_passkey_authentication(&cred, &skr).unwrap();
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
        let text = d.create_text_node("Register");
        let email = d
            .create_input_element()
            .attr("type", "text")
            .attr("placeholder", "Email")
            .attr("required", "true");
        let form = d
            .create_form_element()
            .child(email.clone())
            .child(d.create_element("br"))
            .child(
                d.create_input_element()
                    .attr("type", "submit")
                    .attr("value", "Login"),
            )
            .handler({
                let session = session.clone();
                let email = email.clone();
                move || {
                    if let Err(e) = Self::do_login(session.clone(), &url, &email.input_value()) {
                        session.global().fail(e);
                    }
                }
            });
        let page = d.create_element("div").child(text).child(form);
        Ok(Page::new(session.global().clone(), page))
    }
}
