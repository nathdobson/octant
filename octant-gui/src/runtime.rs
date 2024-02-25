use std::any::Any;
use std::marker::PhantomData;
use std::mem;
use std::sync::{Arc, Weak};

use atomic_refcell::AtomicRefCell;
use futures::{SinkExt, StreamExt};
use weak_table::WeakValueHashMap;

use octant_gui_core::{
    DownMessage, DownMessageList, HandleId, Method, RemoteEvent, TypedHandle, TypeTag,
};

use crate::{EventSource, handle, html_form_element, RenderSink};

struct State {
    buffer: Vec<DownMessage>,
    consumer: RenderSink,
    next_handle: usize,
    handles: WeakValueHashMap<HandleId, Weak<dyn 'static + Any + Send + Sync>>,
}

pub struct Runtime {
    state: AtomicRefCell<State>,
}

impl Runtime {
    pub fn new(consumer: RenderSink) -> Arc<Self> {
        Arc::new(Runtime {
            state: AtomicRefCell::new(State {
                buffer: vec![],
                consumer,
                next_handle: 0,
                handles: WeakValueHashMap::new(),
            }),
        })
    }
    pub async fn handle_events(self: &Arc<Self>, mut events: EventSource) -> anyhow::Result<()> {
        while let Some(event) = events.next().await {
            self.handle_event(event?);
            self.flush().await?;
        }
        Ok(())
    }
    pub fn handle_event(self: &Arc<Self>, event: RemoteEvent) {
        match event {
            RemoteEvent::Submit(handle) => {
                self.handle::<html_form_element::Value>(handle).submit();
            }
        }
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
    pub fn handle<T: HasTypedHandle>(&self, key: TypedHandle<T::TypeTag>) -> Arc<T> {
        Arc::downcast(
            self.state
                .borrow_mut()
                .handles
                .get(&key.0)
                .expect("unknown handle"),
        )
            .expect("not expected type")
    }
}

pub trait HasTypedHandle: handle::Trait {
    type TypeTag: TypeTag;
    fn typed_handle(&self) -> TypedHandle<Self::TypeTag> {
        TypedHandle(handle::Trait::value(self).id(), PhantomData)
    }
}
