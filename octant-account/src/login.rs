use std::{future::Future, rc::Rc, sync::Arc};

use url::Url;
use uuid::Uuid;
use webauthn_rs::prelude::Passkey;

use octant_database::database::ArcDatabase;
use octant_error::{octant_error, OctantResult};
use octant_server::{cookies::CookieRouter, Handler, Page, session::Session};
use octant_web_sys_server::builder::{ElementExt, HtmlFormElementExt, NodeExt};

use crate::{
    AccountTable, build_webauthn, into_auth::IntoAuth, into_octant::IntoOctant, SESSION_COOKIE,
    SessionTable, VerifiedLogin,
};

pub struct LoginHandler {
    pub db: ArcDatabase,
    pub cookie_router: Arc<CookieRouter>,
    pub session_table: Arc<SessionTable>,
}

impl LoginHandler {
    pub fn do_login<'a>(
        self: Arc<Self>,
        session: Rc<Session>,
        url: &'a Url,
        email: &'a str,
    ) -> impl 'a + Future<Output = OctantResult<()>> {
        async move {
            let webauthn = build_webauthn(url)?;
            let passkeys: Vec<Passkey> = {
                let database = self.db.read().await;
                let accounts = database.table_const::<AccountTable>().unwrap();
                let user = accounts
                    .users
                    .get(email)
                    .ok_or_else(|| octant_error!("account does not exist"))?;
                user.passkeys.iter().map(|(k, v)| (***v).clone()).collect()
            };
            let (rcr, skr) = webauthn.start_passkey_authentication(&passkeys)?;
            let options = session.global().new_credential_request_options();
            options.public_key(rcr.public_key.into_octant());
            let cred = session
                .global()
                .window()
                .navigator()
                .credentials()
                .get_with_options(options.clone())
                .await?;
            let cred = cred.into_auth();
            let result = webauthn.finish_passkey_authentication(&cred, &skr).unwrap();
            let session_id = Uuid::new_v4();
            self.session_table.sessions.lock().insert(
                session_id,
                Arc::new(VerifiedLogin {
                    email: email.to_string(),
                }),
            );
            self.cookie_router
                .create(
                    &session,
                    format!(
                        "{}={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}",
                        SESSION_COOKIE,
                        session_id,
                        60 * 60 * 24 * 365
                    ),
                )
                .await?;
            Ok(())
        }
    }
}

impl Handler for LoginHandler {
    fn prefix(&self) -> String {
        "login".to_string()
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
        let form = d
            .create_form_element()
            .child(email.clone())
            .child(d.create_element("br".to_string()))
            .child(
                d.create_input_element()
                    .attr("type", "submit")
                    .attr("value", "Login"),
            )
            .handler(session.global().new_event_listener({
                let session = session.clone();
                move || {
                    let url = url.clone();
                    let session = session.clone();
                    let email = email.clone();
                    let this = self.clone();
                    let spawner = session.global().runtime().spawner().clone();
                    spawner.spawn(async move {
                        this.do_login(session.clone(), &url, &email.input_value())
                            .await
                    });
                }
            }));
        let page = d.create_div_element().child(text).child(form);
        Ok(Page::new(session.global().clone(), page))
    }
}
