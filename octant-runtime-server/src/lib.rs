#![feature(unsize)]
#![feature(macro_metavar_expr)]
#![allow(unused_variables)]
#![deny(unused_must_use)]
#![feature(trait_alias)]
#![feature(arbitrary_self_types)]

extern crate core;
extern crate self as octant_runtime;
#[cfg(side = "client")]
extern crate self as octant_runtime_client;
#[cfg(side = "server")]
extern crate self as octant_runtime_server;

use crate::{
    handle::{RawHandle, TypedHandle},
    immediate_return::AsTypedHandle,
    runtime::Runtime,
};
use marshal::{
    context::Context,
    de::Deserialize,
    decode::{AnyDecoder, Decoder},
    encode::{AnyEncoder, Encoder},
    ser::Serialize,
};
use marshal_bin::{decode::full::BinDecoder, encode::full::BinEncoder};
use marshal_json::{decode::full::JsonDecoder, encode::full::JsonEncoder};
use marshal_pointer::rc_ref::RcRef;
use octant_object::class::Class;
#[cfg(side = "client")]
pub use octant_runtime_derive::PeerNewClient as PeerNew;
#[cfg(side = "server")]
pub use octant_runtime_derive::PeerNewServer as PeerNew;
pub use octant_runtime_derive::*;
use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};
use marshal_pointer::rcf::Rcf;

pub mod handle;
#[doc(hidden)]
pub mod reexports {
    pub use anyhow;
    pub use catalog;
    pub use marshal;
    pub use marshal_object;
    pub use marshal_pointer;
    pub use paste;
    pub use serde;

    pub use octant_error;
    pub use octant_object;
    pub use octant_reffed;
    pub use octant_serde;
}

pub trait OctantDeserialize = Deserialize<JsonDecoder> + Deserialize<BinDecoder>;
pub trait OctantSerialize = Serialize<JsonEncoder> + Serialize<BinEncoder>;

#[cfg_attr(side = "client", path = "client_runtime.rs")]
#[cfg_attr(side = "server", path = "server_runtime.rs")]
pub mod runtime;

mod delete;
pub mod error;
pub mod future_return;
pub mod immediate_return;
pub mod octant_future;
#[cfg_attr(side = "client", path = "client_peer.rs")]
#[cfg_attr(side = "server", path = "server_peer.rs")]
pub mod peer;
pub mod proto;

// pub fn deserialize_object_with<'de, T: ?Sized + Class, D: Deserializer<'de>>(
//     ctx: &DeserializeContext,
//     d: D,
// ) -> Result<Rc2<T>, D::Error> {
//     let runtime = ctx.get::<Rc<Runtime>>().map_err(|e| D::Error::custom(e))?;
//     let handle = TypedHandle::<T>::deserialize(d)?;
//     runtime.lookup(handle).map_err(D::Error::custom)
// }

#[derive(Debug)]
pub enum LookupError {
    NotFound(RawHandle),
    DowncastFailed,
}

impl Display for LookupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LookupError::NotFound(handle) => write!(f, "object {:?} not found", handle),
            LookupError::DowncastFailed => write!(f, "object downcast failed"),
        }
    }
}

impl std::error::Error for LookupError {}

pub trait PeerNew {
    type Builder;
    fn peer_new(builder: Self::Builder) -> Self;
}

impl<T> PeerNew for Rcf<T>
where
    T: ?Sized + Class,
    T::Fields: PeerNew,
{
    type Builder = <T::Fields as PeerNew>::Builder;
    fn peer_new(builder: Self::Builder) -> Self {
        Rcf::<T::Fields>::new(T::Fields::peer_new(builder))
    }
}

pub fn deserialize_peer<'p, 'de, D: Decoder, T: ?Sized + Class>(
    d: AnyDecoder<'p, 'de, D>,
    mut ctx: Context,
) -> anyhow::Result<Rc<T>> {
    let handle = <TypedHandle<T> as Deserialize<D>>::deserialize(d, ctx.reborrow())?;
    let runtime = ctx.get_const::<Rc<Runtime>>()?;
    Ok(runtime.lookup(handle)?.into())
}

pub fn serialize_peer<'w, 'en, E: Encoder, T: ?Sized + AsTypedHandle>(
    peer: &RcRef<T>,
    e: AnyEncoder<'w, 'en, E>,
    ctx: Context,
) -> anyhow::Result<()> {
    <TypedHandle<T> as Serialize<E>>::serialize(&peer.typed_handle(), e, ctx)
}
