use std::marker::Unsize;
use std::rc::Rc;

use serde::Serialize;

use octant_object::class::Class;
use octant_reffed::rc::Rc2;
use octant_serde::DeserializeWith;

use crate::{
    handle::TypedHandle,
    peer::Peer,
    runtime::Runtime,
};
#[cfg(side="server")]
use crate::peer::PeerValue;

pub trait AsTypedHandle: Class {
    fn typed_handle(&self) -> TypedHandle<Self>;
}

impl<T: ?Sized + Class + Unsize<dyn Peer>> AsTypedHandle for T {
    fn typed_handle(&self) -> TypedHandle<Self> {
        TypedHandle::new((self as &dyn Peer).raw_handle())
    }
}

pub trait ImmediateReturn: Sized {
    type Down: Serialize + for<'de> DeserializeWith<'de>;
    #[cfg(side = "server")]
    fn immediate_new(runtime: &Rc<Runtime>) -> (Self, Self::Down);
    #[cfg(side = "client")]
    fn immediate_return(self, runtime: &Rc<Runtime>, down: Self::Down);
}

#[cfg(side="server")]
impl<T: ?Sized + Class + Unsize<dyn Peer>> ImmediateReturn for Rc2<T>
where
    T::Value: Peer + From<PeerValue> + Unsize<T>,
{
    type Down = TypedHandle<T>;
    #[cfg(side = "server")]
    fn immediate_new(runtime: &Rc<Runtime>) -> (Self, Self::Down) {
        let peer: Rc2<T> = runtime.add::<T::Value>(T::Value::from(runtime.add_uninit()));
        let handle = (*peer).typed_handle();
        (peer, handle)
    }

}

#[cfg(side="client")]
impl<T: ?Sized + Class + Unsize<dyn Peer>> ImmediateReturn for Rc2<T>
    where
        T::Value: Peer + Unsize<T>,
{
    type Down = TypedHandle<T>;

    #[cfg(side = "client")]
    fn immediate_return(self, runtime: &Rc<Runtime>, down: Self::Down) {
        runtime.add(down, self);
    }
}

impl ImmediateReturn for () {
    type Down = ();
    #[cfg(side = "server")]
    fn immediate_new(runtime: &Rc<Runtime>) -> ((), ()) {
        ((), ())
    }
    #[cfg(side = "client")]
    fn immediate_return(self, runtime: &Rc<Runtime>, down: ()) {}
}

impl<T1: ImmediateReturn, T2: ImmediateReturn> ImmediateReturn for (T1, T2) {
    type Down = (T1::Down, T2::Down);
    #[cfg(side = "server")]
    fn immediate_new(runtime: &Rc<Runtime>) -> (Self, (T1::Down, T2::Down)) {
        let (t1, t1d) = T1::immediate_new(runtime);
        let (t2, t2d) = T2::immediate_new(runtime);
        ((t1, t2), (t1d, t2d))
    }
    #[cfg(side = "client")]
    fn immediate_return(self, runtime: &Rc<Runtime>, down: (T1::Down, T2::Down)) {
        self.0.immediate_return(runtime, down.0);
        self.1.immediate_return(runtime, down.1);
    }
}
