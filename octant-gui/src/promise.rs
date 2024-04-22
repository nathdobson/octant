use parking_lot::Mutex;
use tokio::sync::oneshot;

use octant_gui_core::{
    Method,
    {PromiseMethod, PromiseTag, PromiseUpMessage},
};
use octant_object::define_class;

use crate::{
    handle, object,
    runtime::{HasLocalType, HasTypedHandle},
};

define_class! {
    #[derive(Debug)]
    pub class extends object{

    }
}

impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: object::Value::new(handle),
        }
    }
}

impl dyn Trait {
    fn invoke(&self, method: PromiseMethod) -> handle::Value {
        (**self).invoke(Method::Promise(self.typed_handle(), method))
    }
    pub fn handle_event(&self, message: PromiseUpMessage) {
        match message {}
    }
}

impl HasTypedHandle for Value {
    type TypeTag = PromiseTag;
}

impl HasLocalType for PromiseTag {
    type Local = dyn Trait;
}

#[derive(Debug)]
pub struct Completable<T> {
    tx: Mutex<Option<oneshot::Sender<T>>>,
    rx: Mutex<Option<oneshot::Receiver<T>>>,
}

impl<T: Send> Completable<T> {
    pub fn new() -> Self {
        let (tx, rx) = oneshot::channel();
        Completable {
            tx: Mutex::new(Some(tx)),
            rx: Mutex::new(Some(rx)),
        }
    }
    pub async fn recv(&self) -> T {
        let recv = self.rx.lock().take().unwrap();
        recv.await.unwrap()
    }
    pub fn send(&self, x: T) {
        self.tx.lock().take().unwrap().send(x).ok();
    }
}
