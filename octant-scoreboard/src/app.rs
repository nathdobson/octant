use octant_gui::event_loop::{Page, Session};
use octant_gui::Global;
use octant_server::Application;
use std::sync::Arc;
use octant_gui::builder::{ElementExt, HtmlFormElementExt};

pub struct ScoreBoardApplication {}

struct ScoreBoardSession {}

impl Application for ScoreBoardApplication {
    fn create_session(&self, _global: Arc<Global>) -> anyhow::Result<Box<dyn Session>> {
        Ok(Box::new(ScoreBoardSession {}))
    }
}

impl Session for ScoreBoardSession {
    fn create_page(&mut self, url: &str, global: Arc<Global>) -> anyhow::Result<Page> {
        let d = global.window().document();
        let text = d.create_input_element().attr("type", "text");
        let form = d
            .create_form_element()
            .child(text.clone())
            .child(d.create_element("br"))
            .child(d.create_input_element().attr("type", "submit"))
            .handler(move || {
                println!("submitted {:?}", text.input_value());
            });
        let page = d
            .create_element("div")
            .child(d.create_text_node(url))
            .child(form);
        Ok(Page::new(global, page))
    }
}
