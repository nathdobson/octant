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

mod arc;
mod de;
mod dict;
mod row;
mod ser;
pub mod tack;
#[cfg(test)]
mod test;
mod util;
mod table;
