use marshal_pointer::Rcf;
use std::{rc::Rc, sync::Arc};
use url::Url;
use webauthn_rs::prelude::{Passkey, Uuid};

use crate::{build_webauthn, into_auth::IntoAuth, into_octant::IntoOctant, Account, AccountTable};
use octant_database::database::ArcDatabase;
use octant_error::{octant_error, Context, OctantResult};
use octant_runtime_server::reexports::marshal_pointer::RcfRef;
use octant_server::{session::Session, PathHandler, UrlPart};
use octant_web_sys_server::{
    builder::{ElementExt, HtmlFormElementExt, NodeExt},
    node::{Node, RcNode},
};

pub struct RegisterApplication {
    pub(crate) db: ArcDatabase,
}

impl RegisterApplication {
    pub async fn do_register(
        self: &Arc<Self>,
        session: Rc<Session>,
        email: String,
        name: String,
    ) -> OctantResult<()> {
        let webauthn = build_webauthn(&session)?;
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

pub struct RegisterHandler {
    app: Arc<RegisterApplication>,
    session: Rc<Session>,
}

impl PathHandler for RegisterHandler {
    fn node(self: Arc<Self>) -> Rcf<dyn Node> {
        todo!()
    }

    fn handle_path(self: Arc<Self>, url: UrlPart) -> OctantResult<()> {
        let app = self.app.clone();
        let d = self.session.global().window().document();
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
            .form_submit_handler({
                let session = self.session.clone();
                let email = email.clone();
                let name = name.clone();
                Box::new(move |()| {
                    let app = app.clone();
                    let session = session.clone();
                    let email = email.clone();
                    let name = name.clone();
                    let spawner = session.global().runtime().spawner().clone();
                    spawner.spawn(async move {
                        app.do_register(
                            session.clone(),
                            (*email.input_value()).clone(),
                            (*name.input_value()).clone(),
                        )
                        .await
                    });
                    Ok(())
                })
            });
        let page = d.create_div_element().child(text).child(form);
        d.body().append_child(page);
        Ok(())
    }
}
