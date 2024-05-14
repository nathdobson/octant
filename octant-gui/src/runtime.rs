use std::{
    any::type_name,
    marker::PhantomData,
    ptr::{DynMetadata, Pointee},
    sync::{Arc, Weak},
};

use atomic_refcell::AtomicRefCell;
use tokio::sync::mpsc::UnboundedSender;
use weak_table::WeakValueHashMap;

use octant_executor::Spawn;
use octant_gui_core::{DownMessage, HandleId, Method, TypedHandle, TypeTag};
use octant_object::cast::downcast_object;

use crate::handle::{Handle, HandleValue};

struct State {
    next_handle: usize,
    handles: WeakValueHashMap<HandleId, Weak<dyn Handle>>,
}

pub struct Runtime {
    state: AtomicRefCell<State>,
    spawn: Arc<Spawn>,
    sink: UnboundedSender<DownMessage>,
}

impl Runtime {
    pub fn new(sink: UnboundedSender<DownMessage>, spawn: Arc<Spawn>) -> Self {
        Runtime {
            state: AtomicRefCell::new(State {
                next_handle: 0,
                handles: WeakValueHashMap::new(),
            }),
            spawn,
            sink,
        }
    }
    pub fn invoke(self: &Arc<Self>, method: Method) -> HandleValue {
        let ref mut this = *self.state.borrow_mut();
        let handle = HandleId(this.next_handle);
        this.next_handle += 1;
        self.sink
            .send(DownMessage::Invoke {
                assign: handle,
                method,
            })
            .ok();
        HandleValue::new(self.clone(), handle)
    }
    pub fn delete(&self, handle: HandleId) {
        self.send(DownMessage::Delete(handle));
    }
    pub fn send(&self, command: DownMessage) {
        self.sink.send(command).ok();
    }
    pub fn spawner(&self) -> &Arc<Spawn> {
        &self.spawn
    }
    // pub async fn flush(&self) -> anyhow::Result<()> {
    //     let ref mut this = *self.state.borrow_mut();
    //     self.sink
    //         .lock()
    //         .flush();
    //         .await?;
    //     Ok(())
    // }
    pub fn add<T: Handle>(&self, value: T) -> Arc<T> {
        let result = Arc::new(value);
        self.state
            .borrow_mut()
            .handles
            .insert(Handle::value(&*result).id(), result.clone());
        result
    }
    pub fn handle<T: HasLocalType>(&self, key: TypedHandle<T>) -> Arc<T::Local> {
        let handle = self
            .state
            .borrow_mut()
            .handles
            .get(&key.0)
            .unwrap_or_else(|| panic!("unknown handle {:?}", key));
        let dhandle = handle.clone();
        downcast_object(handle)
            .unwrap_or_else(|_| panic!("Cannot cast {:?} to {:?}", dhandle, type_name::<T::Local>()))
    }
}

pub trait HasTypedHandle: Handle {
    type TypeTag: TypeTag;
    fn typed_handle(&self) -> TypedHandle<Self::TypeTag> {
        TypedHandle(Handle::value(self).id(), PhantomData)
    }
}

pub trait HasLocalType: TypeTag {
    type Local: ?Sized + Pointee<Metadata = DynMetadata<Self::Local>>;
}
