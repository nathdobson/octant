use std::sync::Arc;

use atomic_refcell::AtomicRefCell;
use url::Url;

use octant_gui::{
    builder::{ElementExt, HtmlFormElementExt},
    event_loop::Page,
};
use octant_server::{
    session::{Session, SessionData},
    Handler,
};

pub struct LoginHandler {}

struct UserId(u64);

#[derive(Default)]
struct LoginState {
    verified_user: Option<UserId>,
}

#[derive(Default)]
struct LoginSession {
    state: AtomicRefCell<LoginState>,
}

impl SessionData for LoginSession {}

impl Handler for LoginHandler {
    fn prefix(&self) -> String {
        "login".to_string()
    }

    fn handle(&self, url: &Url, session: Arc<Session>) -> anyhow::Result<Page> {
        let d = session.global().window().document();
        let text = d.create_text_node("Login");
        let input = d
            .create_input_element()
            .attr("type", "text")
            .attr("placeholder", "Team Name");
        let form = d
            .create_form_element()
            .child(input.clone())
            .child(d.create_element("br"))
            .child(
                d.create_input_element()
                    .attr("type", "submit")
                    .attr("value", "Login"),
            )
            .handler({
                let session = session.clone();
                let text = text.clone();
                move || {
                    let count = {
                        let data = session.data::<LoginSession>();
                        let ref mut state = *data.state.borrow_mut();
                        todo!();
                    };
                }
            });
        let page = d.create_element("div").child(text).child(form);
        Ok(Page::new(session.global().clone(), page))
    }
}
