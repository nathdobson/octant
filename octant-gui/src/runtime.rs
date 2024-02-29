use std::any::type_name;
use std::marker::PhantomData;
use std::mem;
use std::ptr::{DynMetadata, Pointee};
use std::sync::{Arc, Weak};

use atomic_refcell::AtomicRefCell;
use futures::SinkExt;
use weak_table::WeakValueHashMap;

use octant_gui_core::{DownMessage, DownMessageList, HandleId, Method, TypedHandle, TypeTag};
use octant_object::cast::Cast;

use crate::{DownMessageSink, handle};

struct State {
    buffer: Vec<DownMessage>,
    consumer: DownMessageSink,
    next_handle: usize,
    handles: WeakValueHashMap<HandleId, Weak<dyn handle::Trait>>,
}

pub struct Runtime {
    state: AtomicRefCell<State>,
}

impl Runtime {
    pub fn new(consumer: DownMessageSink) -> Arc<Self> {
        Arc::new(Runtime {
            state: AtomicRefCell::new(State {
                buffer: vec![],
                consumer,
                next_handle: 0,
                handles: WeakValueHashMap::new(),
            }),
        })
    }
    pub fn invoke(self: &Arc<Self>, method: Method) -> handle::Value {
        let ref mut this = *self.state.borrow_mut();
        let handle = HandleId(this.next_handle);
        this.next_handle += 1;
        this.buffer.push(DownMessage::Invoke {
            assign: handle,
            method,
        });
        handle::Value::new(self.clone(), handle)
    }
    pub fn delete(&self, handle: HandleId) {
        self.send(DownMessage::Delete(handle));
    }
    pub fn send(&self, command: DownMessage) {
        let ref mut this = *self.state.borrow_mut();
        this.buffer.push(command);
    }
    pub async fn flush(&self) -> anyhow::Result<()> {
        let ref mut this = *self.state.borrow_mut();
        this.consumer
            .send(DownMessageList {
                commands: mem::replace(&mut this.buffer, vec![]),
            })
            .await?;
        Ok(())
    }
    pub fn add<T: handle::Trait>(&self, value: T) -> Arc<T> {
        let result = Arc::new(value);
        self.state
            .borrow_mut()
            .handles
            .insert(handle::Trait::value(&*result).id(), result.clone());
        result
    }
    pub fn handle<T: HasLocalType>(&self, key: TypedHandle<T>) -> Arc<T::Local> {
        let handle = self
            .state
            .borrow_mut()
            .handles
            .get(&key.0)
            .expect("unknown handle");
        let dhandle = handle.clone();
        handle
            .downcast_trait()
            .unwrap_or_else(|| panic!("Cannot cast {:?} to {:?}", dhandle, type_name::<T::Local>()))
    }
}

pub trait HasTypedHandle: handle::Trait {
    type TypeTag: TypeTag;
    fn typed_handle(&self) -> TypedHandle<Self::TypeTag> {
        TypedHandle(handle::Trait::value(self).id(), PhantomData)
    }
}

pub trait HasLocalType: TypeTag {
    type Local: ?Sized + Pointee<Metadata = DynMetadata<Self::Local>>;
}
