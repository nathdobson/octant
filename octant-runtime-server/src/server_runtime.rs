use std::{
    fmt::{Debug, Formatter},
    rc::Rc,
};

use atomic_refcell::AtomicRefCell;
use marshal::context::OwnedContext;
use marshal_pointer::{Rcf, RcfWeak};
use octant_error::OctantResult;
use octant_executor::event_loop::EventSpawn;
use octant_object::{cast::downcast_object, class::Class};
use tokio::sync::mpsc::UnboundedSender;
use weak_table::WeakValueHashMap;

use crate::{
    delete::delete_rpc,
    handle::{RawHandle, TypedHandle},
    peer::{Peer, PeerFields},
    proto::{DownMessage, Proto, UpMessage, UpMessageList},
    LookupError,
};

struct State {
    next_handle: u64,
    handles: WeakValueHashMap<RawHandle, RcfWeak<dyn Peer>>,
}

pub struct Runtime {
    proto: Proto,
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
    pub fn new(
        proto: Proto,
        sink: UnboundedSender<Box<dyn DownMessage>>,
        spawn: Rc<EventSpawn>,
    ) -> Self {
        Runtime {
            proto,
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
    pub fn add<T: Peer>(&self, value: T) -> Rcf<T> {
        let handle = ((&value) as &dyn Peer).raw_handle();
        let result = Rcf::new(value);
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
        delete_rpc(self, handle);
    }
    pub fn run_batch(self: &Rc<Self>, batch: UpMessageList) -> OctantResult<()> {
        let mut ctx = OwnedContext::new();
        ctx.insert_const(self);
        for message in batch.commands {
            let message = self
                .proto
                .deserialize::<Box<dyn UpMessage>>(&message, ctx.borrow())?;
            log::info!("UpMessage: {:#?}", message);
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
    ) -> Result<Rcf<T>, LookupError> {
        Ok(downcast_object(
            self.state
                .borrow()
                .handles
                .get(&handle.raw())
                .ok_or_else(|| LookupError::NotFound(handle.raw()))?,
        )
        .map_err(|_| LookupError::DowncastFailed)?)
    }
    pub fn proto(&self) -> Proto {
        self.proto
    }
}
