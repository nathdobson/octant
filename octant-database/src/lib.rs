#![deny(unused_must_use)]
#![feature(auto_traits)]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(arbitrary_self_types)]
#![feature(hash_raw_entry)]
#![feature(map_try_insert)]
#![feature(new_uninit)]
#![feature(never_type)]
#![feature(unboxed_closures)]
#![feature(dispatch_from_dyn)]
#![feature(debug_closure_helpers)]
#![feature(allocator_api)]
#![feature(raw_ref_op)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![feature(try_blocks)]
#![feature(absolute_path)]

pub mod de;
mod dict;
pub mod tree;
pub mod ser;
#[cfg(test)]
mod test;
mod util;
pub mod forest;
pub mod prim;
pub mod field;
pub mod file;
pub mod derive;
pub mod deserializer_proxy;
pub mod serializer_proxy;
pub mod tack;
pub mod struct_visitor;

pub mod reexports{
    pub use serde;
}
