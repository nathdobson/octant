use std::sync::Arc;

use atomic_refcell::AtomicRefCell;
use octant_account::SessionTable;
use url::Url;

use octant_gui::{
    builder::{ElementExt, HtmlFormElementExt},
    event_loop::Page,
};
use octant_server::{
    cookies::{CookieData, CookieRouter},
    session::{Session, SessionData},
    Handler,
};

pub struct ScoreHandler {
    pub cookie_router: Arc<CookieRouter>,
    pub session_table: Arc<SessionTable>,
}

#[derive(Default)]
struct ScoreState {
    count: usize,
}

#[derive(Default)]
struct ScoreSession {
    state: AtomicRefCell<ScoreState>,
}

impl SessionData for ScoreSession {}

impl Handler for ScoreHandler {
    fn prefix(&self) -> String {
        "score".to_string()
    }

    fn handle(self: Arc<Self>, url: &Url, session: Arc<Session>) -> anyhow::Result<Page> {
        let d = session.global().window().document();
        let text = d.create_text_node(&format!("{:?}", url));
        let input = d.create_input_element().attr("type", "text");
        let form = d
            .create_form_element()
            .child(input.clone())
            .child(d.create_element("br"))
            .child(d.create_input_element().attr("type", "submit"))
            .handler({
                let session = session.clone();
                let text = text.clone();
                move || {
                    let count = {
                        let data = session.data::<ScoreSession>();
                        let ref mut state = *data.state.borrow_mut();
                        state.count += 1;
                        state.count
                    };
                    text.set_node_value(Some(format!("count = {}", count)));
                }
            });
        let page = d.create_element("div").child(text).child(form);
        session.global().runtime().spawner().spawn({
            let session = session.clone();
            async move {
                self.cookie_router.update(&session).await?;
                let user = self.session_table.get(&session);
                log::info!("verified user = {:?}", user);
                Ok(())
            }
        });
        Ok(Page::new(session.global().clone(), page))
    }
}
