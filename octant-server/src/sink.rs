use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{Sink, SinkExt};
use tokio::sync::mpsc::UnboundedReceiver;

use octant_runtime_server::proto::{DownMessage, DownMessageList};
use octant_serde::Format;

pub struct BufferedDownMessageSink {
    source: UnboundedReceiver<Box<dyn DownMessage>>,
    buffer: Vec<Box<dyn DownMessage>>,
    sink: Pin<Box<dyn Sink<DownMessageList, Error = anyhow::Error>>>,
}

impl BufferedDownMessageSink {
    pub fn new(
        source: UnboundedReceiver<Box<dyn DownMessage>>,
        sink: Pin<Box<dyn Sink<DownMessageList, Error = anyhow::Error>>>,
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
                let commands = self
                    .buffer
                    .drain(..)
                    .map(|x| Format::default().serialize(&*x))
                    .collect::<anyhow::Result<Vec<_>>>()?;
                let down = DownMessageList { commands };
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
