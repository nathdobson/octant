use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use atomic_refcell::AtomicRefCell;
use octant_executor::event_loop::EventSpawn;
use octant_object::{cast::downcast_object, class::Class};
use octant_reffed::rc::{Rc2, Weak2};
use octant_serde::DeserializeContext;
use tokio::sync::mpsc::UnboundedSender;
use weak_table::WeakValueHashMap;
use octant_error::OctantResult;

use crate::{
    delete::delete_rpc,
    handle::{RawHandle, TypedHandle},
    LookupError,
    peer::{Peer, PeerFields},
    proto::{DownMessage, UpMessage, UpMessageList},
};

struct State {
    next_handle: u64,
    handles: WeakValueHashMap<RawHandle, Weak2<dyn Peer>>,
}

pub struct Runtime {
    state: AtomicRefCell<State>,
    spawn: Rc<EventSpawn>,
    sink: UnboundedSender<Box<dyn DownMessage>>,
}

impl Debug for Runtime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Runtime").finish_non_exhaustive()
    }
}

impl Runtime {
    pub fn new(sink: UnboundedSender<Box<dyn DownMessage>>, spawn: Rc<EventSpawn>) -> Self {
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
    pub fn spawner(&self) -> &Rc<EventSpawn> {
        &self.spawn
    }
    pub fn add<T: Peer>(&self, value: T) -> Rc2<T> {
        let handle = ((&value) as &dyn Peer).raw_handle();
        log::info!("Adding handle {:?}", handle);
        let result = Rc2::new(value);
        self.state
            .borrow_mut()
            .handles
            .insert(handle, result.clone());
        result
    }
    pub fn add_uninit(self: &Rc<Self>) -> PeerFields {
        let handle;
        {
            let ref mut this = *self.state.borrow_mut();
            handle = RawHandle::new(this.next_handle);
            this.next_handle += 1;
        }
        PeerFields::new(self.clone(), handle)
    }
    pub fn delete(self: &Rc<Self>, handle: RawHandle) {
        log::info!("Deleting handle {:?}", handle);
        delete_rpc(self, handle);
    }
    pub fn run_batch(self: &Rc<Self>, batch: UpMessageList) -> OctantResult<()> {
        let mut ctx = DeserializeContext::new();
        ctx.insert::<Rc<Runtime>>(self.clone());
        for message in batch.commands {
            log::info!("Running up message{:?}", message);
            let message = message.deserialize_with(&ctx)?;
            self.run_message(message)?;
        }
        Ok(())
    }
    pub fn run_message(self: &Rc<Self>, message: Box<dyn UpMessage>) -> OctantResult<()> {
        message.run(self)
    }
    pub fn lookup<T: ?Sized + Class>(
        self: &Rc<Self>,
        handle: TypedHandle<T>,
    ) -> Result<Rc2<T>, LookupError> {
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
