use std::sync::Arc;

use futures::StreamExt;

use octant_gui_core::UpMessage;

use crate::{Global, Node, UpMessageStream};

pub struct EventLoop {
    global: Arc<Global>,
    session: Box<dyn Session>,
    events: UpMessageStream,
    page: Option<Page>,
}

pub trait Session: 'static + Sync + Send {
    fn create_page(&mut self, url: &str, global: Arc<Global>) -> anyhow::Result<Page>;
}

pub struct Page {
    global: Arc<Global>,
    node: Node,
}

impl Page {
    pub fn new(global: Arc<Global>, node: Node) -> Page {
        global.window().document().body().append_child(node.clone());
        Page { global, node }
    }
}

impl Drop for Page {
    fn drop(&mut self) {
        self.global
            .window()
            .document()
            .body()
            .remove_child(self.node.clone());
    }
}
impl EventLoop {
    pub fn new(global: Arc<Global>, events: UpMessageStream, session: Box<dyn Session>) -> Self {
        EventLoop {
            global,
            session,
            events,
            page: None,
        }
    }
    pub async fn handle_events(&mut self) -> anyhow::Result<()> {
        self.global.runtime().flush().await?;
        while let Some(events) = self.events.next().await {
            let events = events?;
            if let Some(events) = events {
                for event in events.commands {
                    self.handle_event(event)?;
                }
                self.global.runtime().flush().await?;
            } else {
                break;
            }
        }
        Ok(())
    }
    pub fn handle_event(&mut self, event: UpMessage) -> anyhow::Result<()> {
        match event {
            UpMessage::HtmlFormElement(form, message) => {
                self.global.runtime().handle(form).handle_event(message);
            }
            UpMessage::HtmlInputElement(input, message) => {
                self.global.runtime().handle(input).handle_event(message);
            }
            UpMessage::VisitPage(page) => {
                self.page = None;
                self.page = Some(self.session.create_page(&page, self.global.clone())?);
            }
        }
        Ok(())
    }
}
