#![deny(unused_must_use)]
#![feature(trait_upcasting)]

use octant_gui::event_loop::{Page, Session};
use std::sync::Arc;

use octant_gui::Global;
use octant_server::{Application, OctantServer, OctantServerOptions};

struct ScoreBoardApplication {}

struct ScoreBoardSession {}

impl Application for ScoreBoardApplication {
    fn create_session(&self, global: Arc<Global>) -> anyhow::Result<Box<dyn Session>> {
        Ok(Box::new(ScoreBoardSession {}))
    }
}

impl Session for ScoreBoardSession {
    fn create_page(&mut self, url: &str, global: Arc<Global>) -> anyhow::Result<Page> {
        let document = global.window().document();
        let page = document.create_element("div");
        let text = document.create_text_node(url);
        page.append_child(text);
        let form = document.create_form_element();
        let text_input = document.create_input_element();
        text_input.set_attribute("type", "text");
        form.append_child(text_input.clone());
        form.append_child(document.create_element("br"));
        let submit = document.create_input_element();
        submit.set_attribute("type", "submit");
        form.append_child(submit);
        form.set_handler({
            move || {
                println!("submitted {:?}", text_input.input_value());
            }
        });
        page.append_child(form);
        Ok(Page::new(global, page))
    }
}

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    let application = ScoreBoardApplication {};
    OctantServer {
        options: OctantServerOptions::from_command_line(),
        application,
    }
    .run()
    .await;
}
