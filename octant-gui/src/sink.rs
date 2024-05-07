use anyhow::Error;
use std::{
    mem,
    pin::Pin,
    task::{ready, Context, Poll},
};

use futures::{Sink, SinkExt};
use tokio::sync::mpsc::UnboundedReceiver;

use octant_gui_core::{DownMessage, DownMessageList};

pub type DownMessageSink = Pin<Box<dyn Send + Sync + Sink<DownMessageList, Error = anyhow::Error>>>;

pub struct BufferedDownMessageSink {
    source: UnboundedReceiver<DownMessage>,
    buffer: Vec<DownMessage>,
    sink: DownMessageSink,
}

impl BufferedDownMessageSink {
    pub fn new(source: UnboundedReceiver<DownMessage>, sink: DownMessageSink) -> Self {
        BufferedDownMessageSink {
            source,
            buffer: vec![],
            sink,
        }
    }
    pub fn poll_flush(&mut self, cx: &mut Context<'_>) -> Poll<anyhow::Result<()>> {
        self.source
            .poll_recv_many(cx, &mut self.buffer, usize::MAX)
            .is_ready();
        let mut pending = false;
        if !self.buffer.is_empty() {
            if let Poll::Ready(()) = self.sink.poll_ready_unpin(cx)? {
                let down = DownMessageList {
                    commands: mem::replace(&mut self.buffer, vec![]),
                };
                log::info!("Sending {:#?}", down);
                self.sink.start_send_unpin(down)?;
            } else {
                pending = true;
            }
        }
        if let Poll::Ready(()) = self.sink.poll_flush_unpin(cx)? {
        } else {
            pending = true;
        }
        if pending {
            Poll::Pending
        } else {
            Poll::Ready(Ok(()))
        }
    }
}
