#[cfg(side = "server")]
use crate::peer::PeerValue;
use crate::{handle::TypedHandle, immediate_return::ImmediateReturn, peer::Peer, runtime::Runtime};
use octant_object::class::Class;
use octant_serde::DeserializeWith;
use serde::Serialize;
use std::{fmt::Debug, marker::Unsize, sync::Arc};
use octant_reffed::Arc2;

pub trait FutureReturn: 'static {
    type Down: 'static + Serialize + for<'de> DeserializeWith<'de> + Send + Sync + Debug;
    type Up: 'static + Serialize + for<'de> DeserializeWith<'de> + Send + Sync + Debug;
    #[cfg(side = "server")]
    type Retain: 'static + Send + Sync + Debug;
    #[cfg(side = "server")]
    fn future_new(runtime: &Arc<Runtime>) -> (Self::Retain, Self::Down);
    #[cfg(side = "client")]
    fn future_produce(self, runtime: &Arc<Runtime>, down: Self::Down) -> Self::Up;
    #[cfg(side = "server")]
    fn future_return(runtime: &Arc<Runtime>, retain: Self::Retain, up: Self::Up) -> Self;
}

#[cfg(side = "server")]
impl<T: ?Sized + Class + Unsize<dyn Peer> + Send + Sync + Debug> FutureReturn for Arc2<T>
where
    T::Value: Peer + From<PeerValue> + Unsize<T>,
{
    type Down = TypedHandle<T>;
    type Up = ();
    type Retain = Self;
    fn future_new(runtime: &Arc<Runtime>) -> (Self::Retain, Self::Down) {
        ImmediateReturn::immediate_new(runtime)
    }
    fn future_return(runtime: &Arc<Runtime>, retain: Self::Retain, up: Self::Up) -> Self {
        retain
    }
}

#[cfg(side = "client")]
impl<T: ?Sized + Class + Unsize<dyn Peer> + Debug> FutureReturn for Arc2<T>
where
    T::Value: Peer + Unsize<T>,
{
    type Down = TypedHandle<T>;
    type Up = ();
    #[cfg(side = "client")]
    fn future_produce(self, runtime: &Arc<Runtime>, down: Self::Down) -> Self::Up {
        self.immediate_return(runtime, down)
    }
}

impl FutureReturn for () {
    type Down = ();
    type Up = ();
    #[cfg(side = "server")]
    type Retain = ();
    #[cfg(side = "server")]
    fn future_new(runtime: &Arc<Runtime>) -> ((), ()) {
        ((), ())
    }
    #[cfg(side = "client")]
    fn future_produce(self, runtime: &Arc<Runtime>, down: Self::Down) -> Self::Up {
        ()
    }
    #[cfg(side = "server")]
    fn future_return(runtime: &Arc<Runtime>, retain: Self::Retain, up: Self::Up) -> Self {
        ()
    }
}

impl<T1: FutureReturn, T2: FutureReturn> FutureReturn for (T1, T2) {
    type Down = (T1::Down, T2::Down);
    type Up = (T1::Up, T2::Up);
    #[cfg(side = "server")]
    type Retain = (T1::Retain, T2::Retain);
    #[cfg(side = "server")]
    fn future_new(runtime: &Arc<Runtime>) -> ((T1::Retain, T2::Retain), (T1::Down, T2::Down)) {
        let (r1, d1) = T1::future_new(runtime);
        let (r2, d2) = T2::future_new(runtime);
        ((r1, r2), (d1, d2))
    }
    #[cfg(side = "client")]
    fn future_produce(self, runtime: &Arc<Runtime>, (d1, d2): Self::Down) -> Self::Up {
        (
            self.0.future_produce(runtime, d1),
            self.1.future_produce(runtime, d2),
        )
    }
    #[cfg(side = "server")]
    fn future_return(runtime: &Arc<Runtime>, (r1, r2): Self::Retain, (u1, u2): Self::Up) -> Self {
        (
            T1::future_return(runtime, r1, u1),
            T2::future_return(runtime, r2, u2),
        )
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
                fn future_new(runtime: &Arc<Runtime>) -> ((), ()) {
                    ((), ())
                }
                #[cfg(side = "client")]
                fn future_produce(self, runtime: &Arc<Runtime>, down: Self::Down) -> Self::Up {
                    self
                }
                #[cfg(side = "server")]
                fn future_return(runtime: &Arc<Runtime>, retain: Self::Retain, up: Self::Up) -> Self {
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
}
