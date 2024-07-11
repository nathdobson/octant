use crate::{
    build_webauthn, into_auth::IntoAuth, into_octant::IntoOctant, register::RegisterComponent,
    style::AccountStyle, AccountTable, SessionTable, VerifiedLogin, SESSION_COOKIE,
};
use marshal_object::reexports::safe_once::cell::OnceCell;
use marshal_pointer::{EmptyRcf, Rcf, RcfRef};
use octant_components::{Component, ComponentBuilder};
use octant_cookies::CookieRouter;
use octant_database::database::ArcDatabase;
use octant_error::{octant_error, OctantResult};
use octant_server::session::Session;
use octant_web_sys_server::{
    builder::{ElementExt, HtmlFormElementExt, NodeExt},
    html_form_element::RcHtmlFormElement,
    html_input_element::RcHtmlInputElement,
    node::{Node, RcNode},
    text::RcText,
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
    style: Rc<AccountStyle>,
}

pub struct LoginComponent {
    db: ArcDatabase,
    sessions: Arc<SessionTable>,
    session: Rc<Session>,
    cookies: Arc<CookieRouter>,
    form: RcHtmlFormElement,
    email_input: RcHtmlInputElement,
    error_text: RcText,
}

impl LoginComponentBuilder {
    pub fn new(
        db: ArcDatabase,
        cookies: Arc<CookieRouter>,
        sessions: Arc<SessionTable>,
        session: Rc<Session>,
        style: Rc<AccountStyle>,
    ) -> Self {
        LoginComponentBuilder {
            db,
            cookies,
            sessions,
            session,
            style,
            path: OnceCell::new(),
        }
    }
}

impl LoginComponent {
    pub async fn login(self: &RcfRef<Self>) -> OctantResult<()> {
        self.error_text.set_node_value("".to_string());
        if let Err(e) = self.login_impl().await {
            self.error_text.set_node_value(format!("Error: {}", e));
        }
        Ok(())
    }
    pub async fn login_impl(self: &RcfRef<Self>) -> OctantResult<()> {
        let email = self.email_input.input_value();
        let webauthn = build_webauthn(&self.session)?;
        let passkeys: Vec<Passkey> = {
            let database = self.db.read().await;
            let accounts = database.table_const::<AccountTable>().unwrap();
            let user = accounts
                .users
                .get(&*email)
                .ok_or_else(|| octant_error!("account does not exist"))?;
            user.passkeys.iter().map(|(k, v)| (***v).clone()).collect()
        };
        let (rcr, skr) = webauthn.start_passkey_authentication(&passkeys)?;
        let options = self.session.global().new_credential_request_options();
        options.public_key(rcr.public_key.into_octant());
        let cred = self
            .session
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
                &self.session,
                format!(
                    "{}={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}",
                    SESSION_COOKIE,
                    session_id,
                    60 * 60 * 24 * 365
                ),
            )
            .await?;
        self.error_text.set_node_value("login successful".to_owned());
        Ok(())
    }
}

impl ComponentBuilder for LoginComponentBuilder {
    fn set_self_path(self: &RcfRef<Self>, path: &str) {
        self.path.set(path.to_owned()).ok().unwrap();
    }

    fn build_component(self: &RcfRef<Self>) -> OctantResult<Rcf<dyn Component>> {
        let this = EmptyRcf::<LoginComponent>::new();
        let d = self.session.global().window().document();
        let error_text;
        let email_input;

        let handler = {
            let this = this.downgrade();
            Box::new(move |()| {
                if let Some(this) = this.upgrade() {
                    this.form.runtime().spawner().spawn({
                        let this = this.clone();
                        async move { this.login().await }
                    });
                }
                Ok(())
            })
        };

        let form = {
            let form = d.create_form_element();
            form.class_list().add(self.style.name());
            form.append_child({
                let header = d.create_heading_element(1);
                header.append_child(d.create_text_node(format!("Login")));
                header
            });
            form.append_child({
                let error = d.create_paragraph_element();
                error.append_child({
                    error_text = d.create_text_node("".to_string());
                    error_text.clone()
                });
                error
            });
            form.append_child(d.create_hr_element());
            form.append_child({
                let email_label = d.create_label_element();
                email_label.append_child(d.create_text_node("Email".to_owned()));

                email_label.append_child({
                    email_input = d.create_input_element();
                    email_label.set_attribute("autocomplete", "email");
                    email_input.set_attribute("id", "email");
                    email_input.set_attribute("type", "text");
                    email_input.set_attribute("placeholder", "Enter Email");
                    email_input.set_attribute("required", "true");
                    email_input.clone()
                });
                email_label
            });
            form.append_child({
                let submit = d.create_input_element();
                submit.set_attribute("type", "submit");
                submit.set_attribute("value", "Login");
                submit
            });
            form.set_form_submit_handler(handler);
            form
        };

        Ok(this.into_strong(LoginComponent {
            db: self.db.clone(),
            sessions: self.sessions.clone(),
            session: self.session.clone(),
            cookies: self.cookies.clone(),
            form,
            email_input,
            error_text,
        }))
    }
}

impl Component for LoginComponent {
    fn node<'a>(self: &'a RcfRef<Self>) -> &'a RcfRef<dyn Node> {
        &*self.form
    }
    fn update_path(self: &RcfRef<Self>, url: &Url) -> OctantResult<()> {
        Ok(())
    }
}
