use std::{
    sync::{Arc, Weak},
};

use atomic_refcell::AtomicRefCell;
use tokio::sync::mpsc::UnboundedSender;
use weak_table::WeakValueHashMap;

use octant_executor::Spawn;
use octant_gui_core::HandleId;

use crate::{
    handle::{Handle, HandleValue},
    ServerDownMessage,
};

struct State {
    next_handle: usize,
    handles: WeakValueHashMap<HandleId, Weak<dyn Handle>>,
}

pub struct Runtime {
    state: AtomicRefCell<State>,
    spawn: Arc<Spawn>,
    sink: UnboundedSender<Box<dyn ServerDownMessage>>,
}

impl Runtime {
    pub fn new(sink: UnboundedSender<Box<dyn ServerDownMessage>>, spawn: Arc<Spawn>) -> Self {
        Runtime {
            state: AtomicRefCell::new(State {
                next_handle: 0,
                handles: WeakValueHashMap::new(),
            }),
            spawn,
            sink,
        }
    }
    pub fn new_send(&self, command: Box<dyn ServerDownMessage>) {
        self.sink.send(command).ok();
    }
    pub fn spawner(&self) -> &Arc<Spawn> {
        &self.spawn
    }
    pub fn add<T: Handle>(&self, value: T) -> Arc<T> {
        let result = Arc::new(value);
        self.state
            .borrow_mut()
            .handles
            .insert(result.handle().id(), result.clone());
        result
    }
    pub fn add_uninit(self: &Arc<Self>) -> HandleValue {
        let handle;
        {
            let ref mut this = *self.state.borrow_mut();
            handle = HandleId(this.next_handle);
            this.next_handle += 1;
        }
        HandleValue::new(self.clone(), handle)
    }
    pub fn delete(self: &Arc<Self>, handle: HandleId) {
        todo!();
    }
}
