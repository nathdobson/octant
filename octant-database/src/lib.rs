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

mod de;
mod dict;
mod tree;
mod ser;
#[cfg(test)]
mod test;
mod util;
mod forest;
mod prim;
mod field;
mod file;

pub use tree::Tree;
pub use forest::Forest;
pub use de::DeserializeUpdate;
pub use de::DeserializeForest;
pub use ser::SerializeForest;
pub use ser::SerializeUpdate;
pub use tree::TreeId;
pub use util::serializer_proxy::SerializerProxy;
pub use util::deserializer_proxy::DeserializerProxy;