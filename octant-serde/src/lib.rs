#![feature(trait_upcasting)]
#![feature(unsize)]
#![allow(unused_variables)]
#![deny(unused_must_use)]

pub use context::*;
pub use deserialize_with::*;
pub use encoded::*;
pub use format::*;
pub use octant_serde_derive::DeserializeWith;
pub use raw_encoded::*;
pub use registry::*;

mod deserialize_with;
mod encoded;
mod format;
mod raw_encoded;
mod context;
mod registry;

pub mod reexports {
    pub use catalog;
    pub use paste;
    pub use serde;
}

