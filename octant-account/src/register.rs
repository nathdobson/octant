use std::{rc::Rc, sync::Arc};

use url::Url;
use webauthn_rs::prelude::{Passkey, Uuid};

use octant_database::database::ArcDatabase;
use octant_error::{Context, octant_error, OctantResult};
use octant_server::{Handler, Page, session::Session};
use octant_web_sys_server::builder::{ElementExt, HtmlFormElementExt, NodeExt};

use crate::{Account, AccountTable, build_webauthn, into_auth::IntoAuth, into_octant::IntoOctant};

pub struct RegisterHandler {
    pub(crate) db: ArcDatabase,
}

impl RegisterHandler {
    pub async fn do_register(
        self: &Arc<Self>,
        session: Rc<Session>,
        url: &Url,
        email: String,
        name: String,
    ) -> OctantResult<()> {
        let webauthn = build_webauthn(url)?;
        let (ccr, skr) =
            webauthn.start_passkey_registration(Uuid::new_v4(), &email, &name, None)?;
        let options = session.global().new_credential_creation_options();
        options.public_key(ccr.public_key.into_octant());
        let cred = session
            .global()
            .window()
            .navigator()
            .credentials()
            .create_with_options(options);

        let this = self.clone();
        let cred = cred.await?.into_auth();
        let result = webauthn
            .finish_passkey_registration(&cred, &skr)
            .context("while verifying passkey")?;
        this.register(email, name, result).await?;
        Ok(())
    }
    async fn register(&self, email: String, name: String, passkey: Passkey) -> OctantResult<()> {
        let mut db = self.db.write().await;
        let accounts = db.table_mut::<AccountTable>();
        if let Some(account) = accounts.users.get(&email) {
            return Err(octant_error!("already registered"));
        }
        let mut account = Account::new(email.clone(), name);
        account.add_passkey(passkey);
        accounts.users.insert(email, account);
        Ok(())
    }
}

impl Handler for RegisterHandler {
    fn prefix(&self) -> String {
        "register".to_string()
    }

    fn handle(self: Arc<Self>, url: &Url, session: Rc<Session>) -> OctantResult<Page> {
        let url = url.clone();
        let d = session.global().window().document();
        let text = d.create_text_node(format!("Register"));
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
            .child(d.create_element("br".to_string()))
            .child(name.clone())
            .child(d.create_element("br".to_string()))
            .child(
                d.create_input_element()
                    .attr("type", "submit")
                    .attr("value", "Register"),
            )
            .handler({
                let session = session.clone();
                let email = email.clone();
                let name = name.clone();
                session.global().new_event_listener({
                    let session = session.clone();
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
                })
            });
        let page = d.create_div_element().child(text).child(form);
        Ok(Page::new(session.global().clone(), page))
    }
}
