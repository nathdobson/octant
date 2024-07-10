use crate::{
    build_webauthn, into_auth::IntoAuth, into_octant::IntoOctant, AccountTable, SessionTable,
    VerifiedLogin, SESSION_COOKIE,
};
use marshal_object::reexports::safe_once::cell::OnceCell;
use marshal_pointer::{Rcf, RcfRef};
use octant_components::{Component, ComponentBuilder};
use octant_cookies::CookieRouter;
use octant_database::database::ArcDatabase;
use octant_error::{octant_error, OctantResult};
use octant_server::session::Session;
use octant_web_sys_server::{
    builder::{ElementExt, HtmlFormElementExt, NodeExt},
    node::{Node, RcNode},
};
use std::{future::Future, rc::Rc, sync::Arc};
use url::Url;
use uuid::Uuid;
use webauthn_rs::prelude::Passkey;

pub struct LoginComponentBuilder {
    db: ArcDatabase,
    cookies: Arc<CookieRouter>,
    sessions: Arc<SessionTable>,
    session: Rc<Session>,
    path: OnceCell<String>,
}

impl LoginComponentBuilder {
    pub fn new(
        db: ArcDatabase,
        cookies: Arc<CookieRouter>,
        sessions: Arc<SessionTable>,
        session: Rc<Session>,
    ) -> Self {
        LoginComponentBuilder {
            db,
            cookies,
            sessions,
            session,
            path: OnceCell::new(),
        }
    }

    pub fn do_login<'a>(
        self: &'a RcfRef<Self>,
        session: Rc<Session>,
        email: &'a str,
    ) -> impl 'a + Future<Output = OctantResult<()>> {
        async move {
            let webauthn = build_webauthn(&session)?;
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
            self.sessions.sessions.lock().insert(
                session_id,
                Arc::new(VerifiedLogin {
                    email: email.to_string(),
                }),
            );
            self.cookies
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

impl ComponentBuilder for LoginComponentBuilder {
    fn set_self_path(self: &RcfRef<Self>, path: &str) {
        self.path.set(path.to_owned()).ok().unwrap();
    }

    fn build_component(self: &RcfRef<Self>) -> OctantResult<Rcf<dyn Component>> {
        let d = self.session.global().window().document();
        let text = d.create_text_node(format!("Login"));
        let email = d
            .create_input_element()
            .attr("type", "text")
            .attr("placeholder", "Email")
            .attr("required", "true");
        let this = self.strong();
        let form = d
            .create_form_element()
            .child(email.clone())
            .child(d.create_br_element())
            .child(
                d.create_input_element()
                    .attr("type", "submit")
                    .attr("value", "Login"),
            )
            .form_submit_handler({
                let session = self.session.clone();
                Box::new(move |()| {
                    let session = session.clone();
                    let email = email.clone();
                    let this = this.clone();
                    let spawner = session.global().runtime().spawner().clone();
                    spawner.spawn(async move {
                        this.do_login(session.clone(), &email.input_value()).await
                    });
                    Ok(())
                })
            });
        let node = d.create_div_element().child(text).child(form);
        Ok(Rcf::new(LoginComponent { node }))
    }
}

pub struct LoginComponent {
    node: RcNode,
}

impl Component for LoginComponent {
    fn node<'a>(self: &'a RcfRef<Self>) -> &'a RcfRef<dyn Node> {
        &*self.node
    }
    fn update_path(self: &RcfRef<Self>, url: &Url) -> OctantResult<()> {
        Ok(())
    }
}
