use std::{collections::HashMap, marker::Unsize, rc::Rc};

use atomic_refcell::AtomicRefCell;
use marshal_pointer::Rcf;
use tokio::sync::mpsc::UnboundedSender;

use octant_error::OctantResult;
use octant_object::{cast::downcast_object, class::Class};

use crate::{
    handle::{RawHandle, TypedHandle},
    peer::{Peer, RcPeer},
    proto::{Proto, UpMessage},
    LookupError,
};

struct State {
    handles: HashMap<RawHandle, RcPeer>,
}

pub struct Runtime {
    proto: Proto,
    state: AtomicRefCell<State>,
    sink: Rc<RuntimeSink>,
}

#[derive(Debug)]
pub struct RuntimeSink {
    sink: UnboundedSender<Box<dyn UpMessage>>,
}

impl Runtime {
    pub fn new(
        proto: Proto,
        sink: UnboundedSender<Box<dyn UpMessage>>,
    ) -> OctantResult<Rc<Runtime>> {
        let runtime = Rc::new(Runtime {
            proto,
            state: AtomicRefCell::new(State {
                handles: HashMap::new(),
            }),
            sink: Rc::new(RuntimeSink { sink }),
        });
        Ok(runtime)
    }

    pub fn add<T: ?Sized + Class + Unsize<dyn Peer>>(
        self: &Rc<Self>,
        assign: TypedHandle<T>,
        value: Rcf<T>,
    ) {
        let value = value as Rcf<dyn Peer>;
        value.init(assign.raw(), self.sink.clone());
        assert!(self
            .state
            .borrow_mut()
            .handles
            .insert(assign.raw(), value)
            .is_none());
    }
    pub fn lookup<T: ?Sized + Class>(&self, handle: TypedHandle<T>) -> Result<Rcf<T>, LookupError> {
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
    pub fn delete(self: &Rc<Self>, handle: RawHandle) {
        self.state.borrow_mut().handles.remove(&handle);
    }
    // pub async fn run_batch(self: &Rc<Self>, messages: DownMessageList) -> OctantResult<()> {
    //     todo!();
    //     // let mut ctx = DeserializeContext::new();
    //     // ctx.insert::<Rc<Runtime>>(self.clone());
    //     // for message in messages.commands {
    //     //     console::info_1(&format!("{:?}", message).into());
    //     //     let message = message.deserialize_with(&ctx)?;
    //     //     self.run_message(message).await?;
    //     // }
    //     Ok(())
    // }
    // async fn run_message(self: &Rc<Self>, message: Box<dyn DownMessage>) -> OctantResult<()> {
    //     message.run(self)?;
    //     Ok(())
    // }
    pub fn sink(&self) -> &Rc<RuntimeSink> {
        &self.sink
    }
    pub fn proto(&self) -> Proto {
        self.proto
    }
}

impl RuntimeSink {
    pub fn send(&self, message: Box<dyn UpMessage>) {
        self.sink.send(message).ok();
    }
}
