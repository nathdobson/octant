use futures::{Sink, SinkExt};
use octant_runtime_server::proto::{DownMessage, DownMessageList};
use std::{
    mem,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct BufferedDownMessageSink {
    source: UnboundedReceiver<Box<dyn DownMessage>>,
    buffer: Vec<Box<dyn DownMessage>>,
    sink: Pin<Box<dyn Send + Sync + Sink<DownMessageList, Error = anyhow::Error>>>,
}

impl BufferedDownMessageSink {
    pub fn new(
        source: UnboundedReceiver<Box<dyn DownMessage>>,
        sink: Pin<Box<dyn Send + Sync + Sink<DownMessageList, Error = anyhow::Error>>>,
    ) -> Self {
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
