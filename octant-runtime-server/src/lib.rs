#![feature(unsize)]
#![feature(macro_metavar_expr)]
#![allow(unused_variables)]
#![deny(unused_must_use)]
#![feature(trait_alias)]

extern crate core;

use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};

use serde::{de::Error, Deserialize, Deserializer};

use octant_object::class::Class;
use octant_reffed::rc::Rc2;
use octant_serde::DeserializeContext;

use crate::{
    handle::{RawHandle, TypedHandle},
    runtime::Runtime,
};

pub mod define_sys_rpc;
pub mod handle;
#[doc(hidden)]
pub mod reexports {
    pub use anyhow;
    pub use catalog;
    pub use paste;
    pub use serde;

    pub use octant_object;
    pub use octant_reffed;
    pub use octant_serde;
}

#[cfg(side = "client")]
pub use octant_runtime_derive::PeerNewClient as PeerNew;
#[cfg(side = "server")]
pub use octant_runtime_derive::PeerNewServer as PeerNew;
pub use octant_runtime_derive::*;

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

pub fn deserialize_object_with<'de, T: ?Sized + Class, D: Deserializer<'de>>(
    ctx: &DeserializeContext,
    d: D,
) -> Result<Rc2<T>, D::Error> {
    let runtime = ctx.get::<Rc<Runtime>>().map_err(|e| D::Error::custom(e))?;
    let handle = TypedHandle::<T>::deserialize(d)?;
    runtime.lookup(handle).map_err(D::Error::custom)
}

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

pub trait PeerNew {
    type Builder;
    fn peer_new(builder: Self::Builder) -> Self;
}
