use std::{panic::AssertUnwindSafe, sync::Arc};

use futures::StreamExt;

use octant_gui_core::UpMessage;
use octant_panic::catch_error;

use crate::{Runtime, UpMessageStream};

pub struct EventLoop {
    runtime: Arc<Runtime>,
    events: UpMessageStream,
}


impl EventLoop {
    pub fn new(runtime: Arc<Runtime>, events: UpMessageStream) -> Self {
        EventLoop {
            runtime,
            events,
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
        todo!()
        // match event {
        //     UpMessage::HtmlFormElement(form, message) => {
        //         self.global.runtime().handle(form).handle_event(message);
        //     }
        //     UpMessage::HtmlInputElement(input, message) => {
        //         self.global.runtime().handle(input).handle_event(message);
        //     }
        //     UpMessage::VisitPage(page) => {
        //         self.page = None;
        //         self.page = Some(self.app.create_page(&page, self.global.clone())?);
        //     }
        //     UpMessage::Credential(credential, data) => {
        //         self.global.runtime().handle(credential).handle_event(data);
        //     }
        //     UpMessage::Promise(promise, message) => {
        //         self.global.runtime().handle(promise).handle_event(message);
        //     } // UpMessage::NewUpMessage(message) => {
        //       //     let handler = UP_MESSAGE_HANDLER_REGISTRY
        //       //         .handlers
        //       //         .get(&(&*message as &dyn Any).type_id())
        //       //         .ok_or_else(|| anyhow!("Missing handler for {:?}", message))?;
        //       //     handler(
        //       //         ServerContext {
        //       //             runtime: self.global.runtime(),
        //       //         },
        //       //         message,
        //       //     )?;
        //       // }
        // }
        // Ok(())
    }
}
