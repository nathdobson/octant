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

pub mod tree;
#[cfg(test)]
mod test;
mod util;
pub mod forest;
pub mod file;
pub mod derive;
pub mod tack;
pub mod de;
pub mod value;
pub mod ser;

pub mod reexports{
    pub use serde;
}
