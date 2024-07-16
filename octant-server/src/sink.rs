use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{Sink, SinkExt};
use tokio::sync::mpsc::UnboundedReceiver;

use octant_runtime_server::{
    proto::{DownMessage, DownMessageList, Proto},
    reexports::{
        marshal::context::OwnedContext,
        octant_error::{OctantError, OctantResult},
    },
};

pub struct BufferedDownMessageSink {
    proto: Proto,
    source: UnboundedReceiver<Box<dyn DownMessage>>,
    buffer: Vec<Box<dyn DownMessage>>,
    sink: Pin<Box<dyn Sink<DownMessageList, Error = OctantError>>>,
}

impl BufferedDownMessageSink {
    pub fn new(
        proto: Proto,
        source: UnboundedReceiver<Box<dyn DownMessage>>,
        sink: Pin<Box<dyn Sink<DownMessageList, Error = OctantError>>>,
    ) -> Self {
        BufferedDownMessageSink {
            proto,
            source,
            buffer: vec![],
            sink,
        }
    }
    pub fn poll_flush(&mut self, cx: &mut Context<'_>) -> Poll<OctantResult<()>> {
        self.source
            .poll_recv_many(cx, &mut self.buffer, usize::MAX)
            .is_ready();
        let mut pending = false;
        if !self.buffer.is_empty() {
            if let Poll::Ready(()) = self.sink.poll_ready_unpin(cx)? {
                let commands = self
                    .buffer
                    .drain(..)
                    .map(|x| {
                        let mut ctx = OwnedContext::new();
                        let output = self.proto.serialize(&x, ctx.borrow())?;
                        Ok(output)
                    })
                    .collect::<OctantResult<Vec<_>>>()?;
                self.sink.start_send_unpin(DownMessageList { commands })?;
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
