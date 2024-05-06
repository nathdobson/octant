use std::sync::Arc;

use anyhow::anyhow;
use tokio::sync::RwLock;
use url::Url;
use webauthn_rs::prelude::Passkey;

use octant_database::{forest::Forest, tree::Tree};
use octant_gui::{
    builder::{ElementExt, HtmlFormElementExt},
    event_loop::Page,
};
use octant_server::{Handler, session::Session};

use crate::{AccountDatabase, build_webauthn, into_auth::IntoAuth, into_octant::IntoOctant};

pub struct LoginHandler {
    pub forest: Arc<RwLock<Forest>>,
    pub accounts: Arc<Tree<AccountDatabase>>,
}

impl LoginHandler {
    pub async fn do_login(
        self: Arc<Self>,
        session: Arc<Session>,
        url: &Url,
        email: &str,
    ) -> anyhow::Result<()> {
        let webauthn = build_webauthn(url)?;
        let passkeys: Vec<Passkey> = {
            let forest = self.forest.read().await;
            let accounts = forest.read(&self.accounts);
            let user = accounts
                .users
                .get(email)
                .ok_or_else(|| anyhow!("account does not exist"))?;
            user.passkeys.iter().map(|(k, v)| (*v).clone()).collect()
        };
        let (rcr, skr) = webauthn.start_passkey_authentication(&passkeys)?;
        let options = session.global().new_credential_request_options();
        options.public_key(rcr.public_key.into_octant());
        let p = session
            .global()
            .window()
            .navigator()
            .credentials()
            .get_with_options(&options);
        let cred = p.get().await?;
        let cred = cred.downcast_credential();
        let cred = cred.materialize();
        let cred = cred.await.into_auth();
        let result = webauthn.finish_passkey_authentication(&cred, &skr).unwrap();
        log::info!("{:?}", result);
        Ok(())
    }
}

impl Handler for LoginHandler {
    fn prefix(&self) -> String {
        "login".to_string()
    }

    fn handle(self: Arc<Self>, url: &Url, session: Arc<Session>) -> anyhow::Result<Page> {
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
                move || {
                    let url = url.clone();
                    let session = session.clone();
                    let email = email.clone();
                    let this = self.clone();
                    tokio::spawn(async move {
                        if let Err(e) = this
                            .do_login(session.clone(), &url, &email.input_value())
                            .await
                        {
                            session.global().fail(e);
                        }
                    });
                }
            });
        let page = d.create_element("div").child(text).child(form);
        Ok(Page::new(session.global().clone(), page))
    }
}
