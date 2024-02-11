#![deny(unused_must_use)]

use std::{iter, mem};
use std::pin::Pin;
use std::sync::Arc;
use atomic_refcell::AtomicRefCell;
use futures::sink::Sink;
use futures::SinkExt;
use serde_json::Value;
use octant_gui_core::{Argument, Command, CommandList, GlobalMethod, Handle, Method, WindowMethod};

type RenderSink = Pin<Box<dyn Send + Sync + Sink<CommandList, Error=anyhow::Error>>>;

struct State {
    buffer: Vec<Command>,
    consumer: RenderSink,
    next_handle: usize,
}

pub struct Root(AtomicRefCell<State>);

pub struct OwnedHandle {
    root: Arc<Root>,
    handle: Handle,
}

impl Root {
    pub fn new(consumer: RenderSink) -> Arc<Self> {
        Arc::new(Root(AtomicRefCell::new(State {
            buffer: vec![],
            consumer,
            next_handle: 0,
        })))
    }
    pub fn invoke(self: &Arc<Self>, method: Method, arguments: Vec<Argument>) -> OwnedHandle {
        let ref mut this = *self.0.borrow_mut();
        let handle = Handle(this.next_handle);
        this.next_handle += 1;
        this.buffer.push(Command::Invoke {
            assign: Some(handle),
            method,
            arguments,
        });
        OwnedHandle { root: self.clone(), handle }
    }
    pub fn delete(&self, handle: Handle) {
        self.send(Command::Delete(handle));
    }
    pub fn send(&self, command: Command) {
        let ref mut this = *self.0.borrow_mut();
        this.buffer.push(command);
    }
    pub async fn flush(&self) -> anyhow::Result<()> {
        let ref mut this = *self.0.borrow_mut();
        this.consumer.send(CommandList {
            commands: mem::replace(&mut this.buffer, vec![])
        }).await?;
        Ok(())
    }
    pub fn log(self: &Arc<Self>, argument: Argument) {
        self.invoke(Method::Log, vec![argument]);
    }
    pub fn window(self: &Arc<Self>) -> Window {
        Window { handle: self.invoke(Method::Global(GlobalMethod::Window), vec![]) }
    }
}

impl Drop for OwnedHandle {
    fn drop(&mut self) {
        self.root.delete(self.handle)
    }
}

impl OwnedHandle {
    pub fn invoke(&self, method: Method, args: Vec<Argument>) -> OwnedHandle {
        self.root.invoke(method, iter::once(Argument::Handle(self.handle)).chain(args.into_iter()).collect())
    }
    pub fn handle(&self) -> Handle {
        self.handle
    }
}

pub struct Window {
    pub handle: OwnedHandle,
}

pub struct Document {
    pub handle: OwnedHandle,
}

impl Window {
    fn invoke(&self, method: WindowMethod, args: Vec<Argument>) -> OwnedHandle {
        self.handle.invoke(Method::Window(method), args)
    }
    pub fn document(&self) -> Document {
        Document { handle: self.invoke(WindowMethod::Document, vec![]) }
    }
}