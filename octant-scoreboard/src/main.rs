#![deny(unused_must_use)]
#![feature(trait_upcasting)]

use std::future::Future;
use std::sync::Arc;

use octant_gui::Global;
use octant_server::{Application, OctantServer, OctantServerOptions};

struct ScoreBoardApplication {}

impl Application for ScoreBoardApplication {
    fn run_handler(&self, global: Arc<Global>) -> impl Future<Output = anyhow::Result<()>> + Send {
        async move {
            let document = global.window().document();
            let body = document.body();
            let text = document.create_text_node("Lorum Ipsum Dolor Sit Amet");
            body.append_child(text);
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
            body.append_child(form);
            global.root().flush().await?;

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
