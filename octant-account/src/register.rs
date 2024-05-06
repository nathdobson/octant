use std::sync::Arc;

use anyhow::{anyhow, Context};
use tokio::sync::RwLock;
use url::Url;
use webauthn_rs::prelude::{Passkey, Uuid};

use octant_database::{forest::Forest, tack::Tack, tree::Tree};
use octant_gui::{
    builder::{ElementExt, HtmlFormElementExt},
    event_loop::Page,
};
use octant_server::{session::Session, Handler};

use crate::{
    build_webauthn, into_auth::IntoAuth, into_octant::IntoOctant, Account, AccountDatabase,
};

pub struct RegisterHandler {
    pub forest: Arc<RwLock<Forest>>,
    pub accounts: Arc<Tree<AccountDatabase>>,
}

impl RegisterHandler {
    pub async fn do_register(
        self: &Arc<Self>,
        session: Arc<Session>,
        url: &Url,
        email: String,
        name: String,
    ) -> anyhow::Result<()> {
        let webauthn = build_webauthn(url)?;
        let (ccr, skr) =
            webauthn.start_passkey_registration(Uuid::new_v4(), &email, &name, None)?;
        let options = session.global().new_credential_creation_options();
        options.public_key(ccr.public_key.into_octant());
        let p = session
            .global()
            .window()
            .navigator()
            .credentials()
            .create_with_options(&options);
        let this = self.clone();

        let cred = p.get().await?;
        let cred = cred.downcast_credential();
        let cred = cred.materialize();
        let cred = cred.await.into_auth();
        let result = webauthn
            .finish_passkey_registration(&cred, &skr)
            .context("while verifying passkey")?;
        this.register(email, name, result).await?;
        Ok(())
    }
    async fn register(&self, email: String, name: String, passkey: Passkey) -> anyhow::Result<()> {
        let read = self.forest.read().await;
        let mut accounts = read.write(&self.accounts);
        let users = accounts.get_mut().users();
        if let Some(account) = users.get(&email) {
            return Err(anyhow!("already registered"));
        }
        let mut account = Account::new(email.clone(), name);
        Tack::new(&mut account).add_passkey(passkey);
        users.insert(email, account);
        Ok(())
    }
}

impl Handler for RegisterHandler {
    fn prefix(&self) -> String {
        "register".to_string()
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
                    let this = self.clone();
                    let session = session.clone();
                    let url = url.clone();
                    let email = email.clone();
                    let name = name.clone();
                    let spawner = session.global().runtime().spawner().clone();
                    spawner.spawn(async move {
                        this.do_register(
                            session.clone(),
                            &url,
                            (*email.input_value()).clone(),
                            (*name.input_value()).clone(),
                        )
                        .await
                    });
                }
            });
        let page = d.create_element("div").child(text).child(form);
        Ok(Page::new(session.global().clone(), page))
    }
}
