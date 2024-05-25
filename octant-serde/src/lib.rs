#![feature(trait_upcasting)]
#![feature(unsize)]
#![allow(unused_variables)]
#![deny(unused_must_use)]

mod deserialize_with;
pub use deserialize_with::*;
mod encoded;
pub use encoded::*;
mod error;
pub use error::*;
mod format;
pub use format::*;
mod raw_encoded;
pub use raw_encoded::*;
mod context;
mod registry;
pub use context::*;

pub use registry::*;

pub mod reexports {
    pub use catalog;
    pub use serde;
    pub use paste;
}

pub use octant_serde_derive::DeserializeWith;