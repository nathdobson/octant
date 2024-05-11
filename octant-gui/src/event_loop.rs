use std::{panic::AssertUnwindSafe, sync::Arc};

use futures::StreamExt;

use octant_gui_core::UpMessage;
use octant_panic::catch_error;
use crate::{Global, UpMessageStream};
use crate::node::{ArcNode, Node};


pub struct EventLoop {
    global: Arc<Global>,
    app: Arc<dyn Application>,
    events: UpMessageStream,
    page: Option<Page>,
}

pub trait Application: 'static + Sync + Send {
    fn create_page(&self, url: &str, global: Arc<Global>) -> anyhow::Result<Page>;
}

pub struct Page {
    global: Arc<Global>,
    node: ArcNode,
}

impl Page {
    pub fn new(global: Arc<Global>, node: ArcNode) -> Page {
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
    pub fn new(global: Arc<Global>, events: UpMessageStream, app: Arc<dyn Application>) -> Self {
        EventLoop {
            global,
            app,
            events,
            page: None,
        }
    }
    // pub async fn handle_events(&mut self) -> anyhow::Result<()> {
    //     if let Err(e) = self.handle_events_impl().await {
    //         self.global
    //             .runtime()
    //             .send(DownMessage::Fail(format!("{:?}", e)));
    //         self.global.runtime().flush().await?;
    //         return Err(e);
    //     }
    //     Ok(())
    // }
    pub async fn handle_events(&mut self) -> anyhow::Result<()> {
        // self.global.runtime().flush().await?;
        while let Some(events) = self.events.next().await {
            let events = events?;
            if let Some(events) = events {
                for event in events.commands {
                    catch_error(AssertUnwindSafe(|| self.handle_event(event)))??
                }
                // self.global.runtime().flush().await?;
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
                self.page = Some(self.app.create_page(&page, self.global.clone())?);
            }
            UpMessage::Credential(credential, data) => {
                self.global.runtime().handle(credential).handle_event(data);
            }
            UpMessage::Promise(promise, message) => {
                self.global.runtime().handle(promise).handle_event(message);
            }
        }
        Ok(())
    }
}
