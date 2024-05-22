use crate::{
    handle::{RawHandle, TypedHandle},
    peer::{ArcPeer, Peer},
    proto::UpMessage,
};
use atomic_refcell::AtomicRefCell;
use octant_object::{cast::downcast_object, class::Class};
use std::{
    collections::HashMap,
    marker::Unsize,
    sync::Arc,
};
use tokio::sync::mpsc::UnboundedSender;
use web_sys::console;
use crate::proto::{DownMessage, DownMessageList};

struct State {
    handles: HashMap<RawHandle, ArcPeer>,
}

pub struct Runtime {
    state: AtomicRefCell<State>,
    sink: UnboundedSender<Box<dyn UpMessage>>,
}

impl Runtime {
    pub fn new(sink: UnboundedSender<Box<dyn UpMessage>>) -> anyhow::Result<Arc<Runtime>> {
        let runtime = Arc::new(Runtime {
            state: AtomicRefCell::new(State {
                handles: HashMap::new(),
            }),
            sink,
        });
        Ok(runtime)
    }

    pub fn add<T: ?Sized + Class + Unsize<dyn Peer>>(
        self: &Arc<Self>,
        assign: TypedHandle<T>,
        value: Arc<T>,
    ) {
        assert!(self
            .state
            .borrow_mut()
            .handles
            .insert(assign.raw(), value)
            .is_none());
    }
    pub fn get<T: ?Sized + Class>(&self, handle: TypedHandle<T>) -> Arc<T> {
        downcast_object(
            self.state
                .borrow()
                .handles
                .get(&handle.raw())
                .expect("unknown handle")
                .clone(),
        )
        .unwrap_or_else(|_| panic!("Wrong class for {:?}", handle))
    }
    pub fn remove(self: &Arc<Self>, handle: RawHandle) {
        self.state.borrow_mut().handles.remove(&handle);
    }
    pub async fn run_batch(self: &Arc<Self>, messages: DownMessageList) -> anyhow::Result<()> {
        for message in messages.commands {
            console::info_1(&format!("{:?}", message).into());
            self.run_message(message).await?;
        }
        Ok(())
    }
    async fn run_message(
        self: &Arc<Self>,
        message: Box<dyn DownMessage>,
    ) -> anyhow::Result<()> {
        message.run(self)?;
        Ok(())
    }
    pub fn send(&self, message: Box<dyn UpMessage>) {
        self.sink.send(message).ok();
    }
}

