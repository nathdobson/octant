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
pub mod de;
pub mod derive;
pub mod file;
pub mod forest;
pub mod ser;
pub mod tack;
#[cfg(test)]
mod test;
pub mod tree;
mod unique_arc;
pub mod value;
pub mod json;

pub mod reexports {
    pub use serde;
}
