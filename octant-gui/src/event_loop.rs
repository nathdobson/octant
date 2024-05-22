use std::{panic::AssertUnwindSafe, sync::Arc};

use futures::StreamExt;

use octant_panic::catch_error;

use crate::{Runtime, ServerUpMessage, UpMessageStream};

pub struct EventLoop {
    runtime: Arc<Runtime>,
    events: UpMessageStream,
}

impl EventLoop {
    pub fn new(runtime: Arc<Runtime>, events: UpMessageStream) -> Self {
        EventLoop { runtime, events }
    }
    fn handle_event(&mut self, event: Box<dyn ServerUpMessage>) -> anyhow::Result<()> {
        todo!();
    }
    pub async fn handle_events(&mut self) -> anyhow::Result<()> {
        while let Some(events) = self.events.next().await {
            let events = events?;
            if let Some(events) = events {
                for event in events.commands {
                    catch_error(AssertUnwindSafe(|| self.handle_event(event)))??
                }
            } else {
                break;
            }
        }
        Ok(())
    }
}
