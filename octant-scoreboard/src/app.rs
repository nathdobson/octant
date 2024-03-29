use std::sync::Arc;

use atomic_refcell::AtomicRefCell;

use octant_gui::builder::{ElementExt, HtmlFormElementExt};
use octant_gui::event_loop::Page;
use octant_server::Handler;
use octant_server::session::{Session, SessionData};

pub struct ScoreHandler {}

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
    fn handle(&self, url: &str, session: Arc<Session>) -> anyhow::Result<Page> {
        let d = session.global().window().document();
        let text = d.create_text_node(url);
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
        Ok(Page::new(session.global().clone(), page))
    }
}
