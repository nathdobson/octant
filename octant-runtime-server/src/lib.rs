#![feature(unsize)]
#![feature(macro_metavar_expr)]
#![allow(unused_variables)]
#![deny(unused_must_use)]

use crate::{
    handle::{RawHandle, TypedHandle},
    runtime::Runtime,
};
use octant_object::class::Class;
use octant_serde::TypeMap;
use serde::{de::Error, Deserialize, Deserializer};
use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

pub mod define_sys_class;
pub mod define_sys_rpc;
pub mod handle;
#[doc(hidden)]
pub mod reexports {
    pub use anyhow;
    pub use catalog;
    pub use paste;
    pub use serde;

    pub use octant_object;
    pub use octant_serde;
}

#[cfg_attr(side = "client", path = "client_runtime.rs")]
#[cfg_attr(side = "server", path = "server_runtime.rs")]
pub mod runtime;

mod delete;
#[cfg_attr(side = "client", path = "client_peer.rs")]
#[cfg_attr(side = "server", path = "server_peer.rs")]
pub mod peer;
pub mod proto;

pub fn deserialize_object_with<'de, T: ?Sized + Class, D: Deserializer<'de>>(
    ctx: &TypeMap,
    d: D,
) -> Result<Arc<T>, D::Error> {
    let runtime = ctx
        .get::<Arc<Runtime>>()
        .ok_or_else(|| D::Error::custom("missing runtime in context"))?;
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
