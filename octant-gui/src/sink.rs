use futures::{Sink, SinkExt};
use octant_gui_core::{DownMessage, DownMessageList};
use parking_lot::Mutex;
use std::{mem, pin::Pin, task::Context};
use std::task::{Poll, ready};

pub type DownMessageSink = Pin<Box<dyn Send + Sync + Sink<DownMessageList, Error = anyhow::Error>>>;

pub struct BufferedDownMessageSink {
    buffer: Vec<DownMessage>,
    sink: DownMessageSink,
}

impl BufferedDownMessageSink {
    pub fn new(sink: DownMessageSink) -> Self {
        BufferedDownMessageSink {
            buffer: vec![],
            sink,
        }
    }
    pub fn send(&mut self, item: DownMessage) {
        self.buffer.push(item);
    }
    pub fn poll_flush(&mut self, cx: &mut Context<'_>) -> Poll<anyhow::Result<()>> {
        ready!(self.sink.poll_ready_unpin(cx))?;
        self.sink.start_send_unpin(DownMessageList{commands:mem::replace(&mut self.buffer,vec![])})?;
        Poll::Ready(Ok(()))
    }
}
