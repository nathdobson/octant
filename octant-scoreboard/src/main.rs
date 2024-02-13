#![deny(unused_must_use)]

use std::future::Future;
use std::sync::Arc;

use octant_gui::Root;
use octant_server::{Application, OctantServer, OctantServerOptions};

struct ScoreBoardApplication {}

impl Application for ScoreBoardApplication {
    fn run_handler(&self, root: Arc<Root>) -> impl Future<Output=anyhow::Result<()>> + Send {
        async move {
            let document = root.window().document();
            let body = document.body();
            let text = document.create_text_node("Lorum Ipsum Dolor Sit Amet");
            body.append_child(&text);
            let form = document.create_element("form");
            body.append_child(&form);
            let text_input = document.create_element("input");
            form.append_child(&text_input);
            text_input.set_attribute("type", "text");
            let submit = document.create_element("input");
            form.append_child(&submit);
            submit.set_attribute("type", "submit");

            root.flush().await?;
            anyhow::Result::Ok(())
        }
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
