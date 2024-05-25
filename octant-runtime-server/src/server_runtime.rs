use crate::{
    delete::delete_rpc,
    handle::{RawHandle, TypedHandle},
    peer::{Peer, PeerValue},
    proto::{DownMessage, UpMessageList},
    LookupError,
};
use atomic_refcell::AtomicRefCell;
use octant_executor::Spawn;
use octant_object::{cast::downcast_object, class::Class};
use octant_serde::DeserializeContext;
use std::{
    fmt::{Debug, Formatter},
    sync::{Arc, Weak},
};
use tokio::sync::mpsc::UnboundedSender;
use weak_table::WeakValueHashMap;
use octant_reffed::{Arc2, Weak2};
use crate::proto::UpMessage;

struct State {
    next_handle: u64,
    handles: WeakValueHashMap<RawHandle, Weak2<dyn Peer>>,
}

pub struct Runtime {
    state: AtomicRefCell<State>,
    spawn: Arc<Spawn>,
    sink: UnboundedSender<Box<dyn DownMessage>>,
}

impl Debug for Runtime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Runtime").finish_non_exhaustive()
    }
}

impl Runtime {
    pub fn new(sink: UnboundedSender<Box<dyn DownMessage>>, spawn: Arc<Spawn>) -> Self {
        Runtime {
            state: AtomicRefCell::new(State {
                next_handle: 0,
                handles: WeakValueHashMap::new(),
            }),
            spawn,
            sink,
        }
    }
    pub fn send(&self, command: Box<dyn DownMessage>) {
        self.sink.send(command).ok();
    }
    pub fn spawner(&self) -> &Arc<Spawn> {
        &self.spawn
    }
    pub fn add<T: Peer>(&self, value: T) -> Arc2<T> {
        let handle = ((&value) as &dyn Peer).raw_handle();
        let result = Arc2::new(value);
        self.state
            .borrow_mut()
            .handles
            .insert(handle, result.clone());
        result
    }
    pub fn add_uninit(self: &Arc<Self>) -> PeerValue {
        let handle;
        {
            let ref mut this = *self.state.borrow_mut();
            handle = RawHandle::new(this.next_handle);
            this.next_handle += 1;
        }
        PeerValue::new(self.clone(), handle)
    }
    pub fn delete(self: &Arc<Self>, handle: RawHandle) {
        delete_rpc(self, handle);
    }
    pub fn run_batch(self: &Arc<Self>, batch: UpMessageList) -> anyhow::Result<()> {
        let mut ctx = DeserializeContext::new();
        ctx.insert::<Arc<Runtime>>(self.clone());
        for message in batch.commands {
            log::info!("Running up message{:?}", message);
            let message = message.deserialize_with(&ctx)?;
            self.run_message(message)?;
        }
        Ok(())
    }
    pub fn run_message(self: &Arc<Self>, message: Box<dyn UpMessage>) -> anyhow::Result<()> {
        message.run(self)
    }
    pub fn lookup<T: ?Sized + Class>(
        self: &Arc<Self>,
        handle: TypedHandle<T>,
    ) -> Result<Arc2<T>, LookupError> {
        Ok(downcast_object(
            self.state
                .borrow()
                .handles
                .get(&handle.raw())
                .ok_or_else(|| LookupError::NotFound(handle.raw()))?,
        )
        .map_err(|_| LookupError::DowncastFailed)?)
    }
}
