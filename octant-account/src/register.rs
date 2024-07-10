use std::rc::Rc;

use crate::{
    build_webauthn, into_auth::IntoAuth, into_octant::IntoOctant, style::AccountStyle, Account,
    AccountTable,
};
use marshal_pointer::{EmptyRcf, Rcf, RcfRef};
use octant_components::{Component, ComponentBuilder};
use octant_database::database::ArcDatabase;
use octant_error::{octant_error, Context, OctantResult};
use octant_server::session::Session;
use octant_web_sys_server::{
    html_form_element::RcHtmlFormElement, html_input_element::RcHtmlInputElement, node::Node,
    text::RcText,
};
use safe_once::cell::OnceCell;
use url::Url;
use webauthn_rs::prelude::Uuid;

pub struct RegisterComponentBuilder {
    db: ArcDatabase,
    self_path: OnceCell<String>,
    session: Rc<Session>,
    style: Rc<AccountStyle>,
}

impl RegisterComponentBuilder {
    pub fn new(db: ArcDatabase, session: Rc<Session>, style: Rc<AccountStyle>) -> Self {
        RegisterComponentBuilder {
            db,
            self_path: OnceCell::new(),
            session,
            style,
        }
    }
}

pub struct RegisterComponent {
    db: ArcDatabase,
    session: Rc<Session>,
    form: RcHtmlFormElement,
    name_input: RcHtmlInputElement,
    email_input: RcHtmlInputElement,
    error_text: RcText,
}

impl RegisterComponent {
    async fn register(self: &RcfRef<Self>) -> OctantResult<()> {
        self.error_text.set_node_value("".to_string());
        if let Err(e) = self.register_impl().await {
            self.error_text.set_node_value(format!("Error: {}", e));
        }
        Ok(())
    }
    async fn register_impl(self: &RcfRef<Self>) -> OctantResult<()> {
        let email = self.email_input.input_value();
        let name = self.name_input.input_value();
        let webauthn = build_webauthn(&self.session)?;
        let (ccr, skr) =
            webauthn.start_passkey_registration(Uuid::new_v4(), &email, &name, None)?;
        let options = self.session.global().new_credential_creation_options();
        options.public_key(ccr.public_key.into_octant());
        let cred = self
            .session
            .global()
            .window()
            .navigator()
            .credentials()
            .create_with_options(options)
            .await?
            .into_auth();
        let passkey = webauthn
            .finish_passkey_registration(&cred, &skr)
            .context("while verifying passkey")?;
        let mut db = self.db.write().await;
        let accounts = db.table_mut::<AccountTable>();
        if let Some(account) = accounts.users.get(&*email) {
            return Err(octant_error!("account already registered"));
        }
        let mut account = Account::new((*email).clone(), (*name).clone());
        account.add_passkey(passkey);
        accounts.users.insert((*email).clone(), account);
        Ok(())
    }
}

impl ComponentBuilder for RegisterComponentBuilder {
    fn set_self_path(self: &RcfRef<Self>, path: &str) {
        self.self_path.set(path.to_owned()).ok().unwrap();
    }

    fn build_component(self: &RcfRef<Self>) -> OctantResult<Rcf<dyn Component>> {
        let this = EmptyRcf::<RegisterComponent>::new();
        let d = self.session.global().window().document();
        let error_text;
        let email_input;
        let name_input;

        let handler = {
            let this = this.downgrade();
            Box::new(move |()| {
                if let Some(this) = this.upgrade() {
                    this.form.runtime().spawner().spawn({
                        let this = this.clone();
                        async move { this.register().await }
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
                header.append_child(d.create_text_node(format!("Register")));
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
                    email_input.set_attribute("type", "text");
                    email_input.set_attribute("placeholder", "Enter Email");
                    email_input.set_attribute("required", "true");
                    email_input.clone()
                });
                email_label
            });
            form.append_child({
                let name_label = d.create_label_element();
                name_label.append_child(d.create_text_node("Team Name".to_owned()));
                name_label.append_child({
                    name_input = d.create_input_element();
                    name_input.set_attribute("type", "text");
                    name_input.set_attribute("placeholder", "Enter Team Name");
                    name_input.set_attribute("required", "true");
                    name_input.clone()
                });
                name_label
            });
            form.append_child({
                let submit = d.create_input_element();
                submit.set_attribute("type", "submit");
                submit.set_attribute("value", "Register");
                submit
            });
            form.set_form_submit_handler(handler);
            form
        };

        Ok(this.into_strong(RegisterComponent {
            db: self.db.clone(),
            session: self.session.clone(),
            form,
            email_input,
            name_input,
            error_text,
        }))
    }
}

impl Component for RegisterComponent {
    fn node<'a>(self: &'a RcfRef<Self>) -> &'a RcfRef<dyn Node> {
        &*self.form
    }

    fn update_path(self: &RcfRef<Self>, url: &Url) -> OctantResult<()> {
        Ok(())
    }
}
