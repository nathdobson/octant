use parking_lot::Mutex;
use tokio::sync::oneshot;

use crate::object::{Object, ObjectValue};
use octant_gui_core::{Error, Method, PromiseMethod, PromiseTag, PromiseUpMessage};
use octant_object::define_class;
use crate::any_value::{AnyValueValue, ArcAnyValue};
use crate::handle::HandleValue;
use crate::runtime::{HasLocalType, HasTypedHandle};

define_class! {
    #[derive(Debug)]
    pub class Promise extends Object{
        completable: Completable<Result<(), Error>>,
    }
}

impl PromiseValue {
    pub fn new(handle: HandleValue) -> Self {
        PromiseValue {
            parent: ObjectValue::new(handle),
            completable: Completable::new(),
        }
    }
}

impl dyn Promise {
    fn invoke(&self, method: PromiseMethod) -> HandleValue {
        (**self).invoke(Method::Promise(self.typed_handle(), method))
    }
    pub fn handle_event(&self, message: PromiseUpMessage) {
        log::info!("promise done");
        match message {
            PromiseUpMessage::Done(x) => self.completable.send(x),
        }
    }
    pub fn wait(&self) {
        self.invoke(PromiseMethod::Wait);
    }
    pub async fn get(&self) -> Result<ArcAnyValue, Error> {
        self.completable.recv().await?;
        log::info!("starting get");
        Ok(self
            .runtime()
            .add(AnyValueValue::new(self.invoke(PromiseMethod::Get))))
    }
}

impl HasTypedHandle for PromiseValue {
    type TypeTag = PromiseTag;
}

impl HasLocalType for PromiseTag {
    type Local = dyn Promise;
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
