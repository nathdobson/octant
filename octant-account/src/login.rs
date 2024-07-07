use crate::{
    build_webauthn, into_auth::IntoAuth, into_octant::IntoOctant, AccountTable, SessionTable,
    VerifiedLogin, SESSION_COOKIE,
};
use marshal_pointer::Rcf;
use octant_cookies::CookieRouter;
use octant_database::database::ArcDatabase;
use octant_error::{octant_error, OctantResult};
use octant_runtime_server::reexports::marshal_pointer::RcfRef;
use octant_server::{session::Session, PathHandler, UrlPart};
use octant_web_sys_server::{
    builder::{ElementExt, HtmlFormElementExt, NodeExt},
    node::{Node, RcNode},
};
use std::{future::Future, rc::Rc, sync::Arc};
use url::Url;
use uuid::Uuid;
use webauthn_rs::prelude::Passkey;

pub struct LoginApplication {
    pub(crate) db: ArcDatabase,
    pub(crate) cookies: Arc<CookieRouter>,
    pub(crate) sessions: Arc<SessionTable>,
}

impl LoginApplication {
    pub fn do_login<'a>(
        self: Arc<Self>,
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

pub struct LoginHandler {
    app: Arc<LoginApplication>,
    session: Rc<Session>,
}

impl PathHandler for LoginHandler {
    fn node(self: Arc<Self>) -> Rcf<dyn Node> {
        todo!()
    }

    fn handle_path(self: Arc<Self>, url: UrlPart) -> OctantResult<()> {
        let d = self.session.global().window().document();
        let text = d.create_text_node(format!("Register"));
        let email = d
            .create_input_element()
            .attr("type", "text")
            .attr("placeholder", "Email")
            .attr("required", "true");
        let app = self.app.clone();
        let form = d
            .create_form_element()
            .child(email.clone())
            .child(d.create_element("br".to_string()))
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
                    let app = app.clone();
                    let spawner = session.global().runtime().spawner().clone();
                    spawner.spawn(async move {
                        app.do_login(session.clone(), &email.input_value()).await
                    });
                    Ok(())
                })
            });
        let page = d.create_div_element().child(text).child(form);
        d.body().append_child(page);
        Ok(())
    }
}
