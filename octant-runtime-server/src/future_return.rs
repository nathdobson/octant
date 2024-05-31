use std::{fmt::Debug, marker::Unsize, rc::Rc};

use serde::Serialize;

use octant_error::OctantError;
use octant_object::class::Class;
use octant_reffed::rc::Rc2;
use octant_serde::DeserializeWith;

#[cfg(side = "server")]
use crate::peer::PeerFields;
#[cfg(side = "server")]
use crate::PeerNew;
use crate::{handle::TypedHandle, immediate_return::ImmediateReturn, peer::Peer, runtime::Runtime};

pub trait FutureReturn: 'static {
    type Down: 'static + Serialize + for<'de> DeserializeWith<'de> + Debug;
    type Up: 'static + Serialize + for<'de> DeserializeWith<'de> + Debug;
    #[cfg(side = "server")]
    type Retain: 'static + Debug;
    #[cfg(side = "server")]
    fn future_new(runtime: &Rc<Runtime>) -> (Self::Retain, Self::Down);
    #[cfg(side = "client")]
    fn future_produce(self, runtime: &Rc<Runtime>, down: Self::Down) -> Self::Up;
    #[cfg(side = "server")]
    fn future_return(runtime: &Rc<Runtime>, retain: Self::Retain, up: Self::Up) -> Self;
}

#[cfg(side = "server")]
impl<T: ?Sized + Class + Unsize<dyn Peer> + Debug> FutureReturn for Rc2<T>
where
    T::Fields: Peer + PeerNew<Builder =PeerFields> + Unsize<T>,
{
    type Down = TypedHandle<T>;
    type Up = ();
    type Retain = Self;
    fn future_new(runtime: &Rc<Runtime>) -> (Self::Retain, Self::Down) {
        ImmediateReturn::immediate_new(runtime)
    }
    fn future_return(runtime: &Rc<Runtime>, retain: Self::Retain, up: Self::Up) -> Self {
        retain
    }
}

#[cfg(side = "client")]
impl<T: ?Sized + Class + Unsize<dyn Peer> + Debug> FutureReturn for Rc2<T>
where
    T::Fields: Peer + Unsize<T>,
{
    type Down = TypedHandle<T>;
    type Up = ();
    #[cfg(side = "client")]
    fn future_produce(self, runtime: &Rc<Runtime>, down: Self::Down) -> Self::Up {
        self.immediate_return(runtime, down)
    }
}

impl FutureReturn for () {
    type Down = ();
    type Up = ();
    #[cfg(side = "server")]
    type Retain = ();
    #[cfg(side = "server")]
    fn future_new(runtime: &Rc<Runtime>) -> ((), ()) {
        ((), ())
    }
    #[cfg(side = "client")]
    fn future_produce(self, runtime: &Rc<Runtime>, down: Self::Down) -> Self::Up {
        ()
    }
    #[cfg(side = "server")]
    fn future_return(runtime: &Rc<Runtime>, retain: Self::Retain, up: Self::Up) -> Self {
        ()
    }
}

impl<T1: FutureReturn, T2: FutureReturn> FutureReturn for (T1, T2) {
    type Down = (T1::Down, T2::Down);
    type Up = (T1::Up, T2::Up);
    #[cfg(side = "server")]
    type Retain = (T1::Retain, T2::Retain);
    #[cfg(side = "server")]
    fn future_new(runtime: &Rc<Runtime>) -> ((T1::Retain, T2::Retain), (T1::Down, T2::Down)) {
        let (r1, d1) = T1::future_new(runtime);
        let (r2, d2) = T2::future_new(runtime);
        ((r1, r2), (d1, d2))
    }
    #[cfg(side = "client")]
    fn future_produce(self, runtime: &Rc<Runtime>, (d1, d2): Self::Down) -> Self::Up {
        (
            self.0.future_produce(runtime, d1),
            self.1.future_produce(runtime, d2),
        )
    }
    #[cfg(side = "server")]
    fn future_return(runtime: &Rc<Runtime>, (r1, r2): Self::Retain, (u1, u2): Self::Up) -> Self {
        (
            T1::future_return(runtime, r1, u1),
            T2::future_return(runtime, r2, u2),
        )
    }
}

impl<T: FutureReturn, E: FutureReturn> FutureReturn for Result<T, E> {
    type Down = (T::Down, E::Down);
    type Up = Result<T::Up, E::Up>;
    #[cfg(side = "server")]
    type Retain = (T::Retain, E::Retain);

    #[cfg(side = "server")]
    fn future_new(runtime: &Rc<Runtime>) -> (Self::Retain, Self::Down) {
        let (tr, td) = T::future_new(runtime);
        let (er, ed) = E::future_new(runtime);
        ((tr, er), (td, ed))
    }
    #[cfg(side = "client")]
    fn future_produce(self, runtime: &Rc<Runtime>, (td, ed): Self::Down) -> Self::Up {
        match self {
            Ok(t) => Ok(t.future_produce(runtime, td)),
            Err(e) => Err(e.future_produce(runtime, ed)),
        }
    }
    #[cfg(side = "server")]
    fn future_return(runtime: &Rc<Runtime>, (tr, er): Self::Retain, up: Self::Up) -> Self {
        match up {
            Ok(t) => Ok(T::future_return(runtime, tr, t)),
            Err(e) => Err(E::future_return(runtime, er, e)),
        }
    }
}

#[derive(Debug)]
pub struct DataReturn<T>(T);
impl<T> DataReturn<T> {
    pub fn new(x: T) -> Self {
        DataReturn(x)
    }
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: 'static + Debug + Serialize + for<'de> DeserializeWith<'de>> FutureReturn
    for DataReturn<T>
{
    type Down = ();
    type Up = T;
    #[cfg(side = "server")]
    type Retain = ();

    #[cfg(side = "server")]
    fn future_new(runtime: &Rc<Runtime>) -> (Self::Retain, Self::Down) {
        ((), ())
    }

    #[cfg(side = "client")]
    fn future_produce(self, runtime: &Rc<Runtime>, down: Self::Down) -> Self::Up {
        self.0
    }
    #[cfg(side = "server")]
    fn future_return(runtime: &Rc<Runtime>, _: Self::Retain, up: Self::Up) -> Self {
        DataReturn(up)
    }
}

macro_rules! future_return_simple {
    ($($type:ty;)*) => {
        $(
            impl FutureReturn for $type {
                type Down = ();
                type Up = Self;
                #[cfg(side = "server")]
                type Retain = ();
                #[cfg(side = "server")]
                fn future_new(runtime: &Rc<Runtime>) -> ((), ()) {
                    ((), ())
                }
                #[cfg(side = "client")]
                fn future_produce(self, runtime: &Rc<Runtime>, down: Self::Down) -> Self::Up {
                    self
                }
                #[cfg(side = "server")]
                fn future_return(runtime: &Rc<Runtime>, retain: Self::Retain, up: Self::Up) -> Self {
                    up
                }
            }
        )*
    };
}

future_return_simple! {
    bool;
    u8;u16;u32;u64;u128;
    i8;i16;i32;i64;i128;
    f32;f64;
    String;
    OctantError;
}
