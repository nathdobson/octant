use crate::{
    handle::{RawHandle, TypedHandle},
    peer::{ArcPeer, Peer},
    proto::{DownMessage, DownMessageList, UpMessage},
    LookupError,
};
use atomic_refcell::AtomicRefCell;
use octant_object::{cast::downcast_object, class::Class};
use octant_serde::DeserializeContext;
use std::{collections::HashMap, marker::Unsize, sync::Arc};
use tokio::sync::mpsc::UnboundedSender;
use web_sys::console;
use octant_reffed::Arc2;

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
        value: Arc2<T>,
    ) {
        let value = value as Arc2<dyn Peer>;
        value.set_handle(assign.raw());
        assert!(self
            .state
            .borrow_mut()
            .handles
            .insert(assign.raw(), value)
            .is_none());
    }
    pub fn lookup<T: ?Sized + Class>(&self, handle: TypedHandle<T>) -> Result<Arc2<T>, LookupError> {
        Ok(downcast_object(
            self.state
                .borrow()
                .handles
                .get(&handle.raw())
                .cloned()
                .ok_or_else(|| LookupError::NotFound(handle.raw()))?,
        )
        .map_err(|_| LookupError::DowncastFailed)?)
    }
    pub fn delete(self: &Arc<Self>, handle: RawHandle) {
        self.state.borrow_mut().handles.remove(&handle);
    }
    pub async fn run_batch(self: &Arc<Self>, messages: DownMessageList) -> anyhow::Result<()> {
        let mut ctx = DeserializeContext::new();
        ctx.insert::<Arc<Runtime>>(self.clone());
        for message in messages.commands {
            console::info_1(&format!("{:?}", message).into());
            let message = message.deserialize_with(&ctx)?;
            self.run_message(message).await?;
        }
        Ok(())
    }
    async fn run_message(self: &Arc<Self>, message: Box<dyn DownMessage>) -> anyhow::Result<()> {
        message.run(self)?;
        Ok(())
    }
    pub fn send(&self, message: Box<dyn UpMessage>) {
        self.sink.send(message).ok();
    }
}
