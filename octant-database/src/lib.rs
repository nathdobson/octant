#![deny(unused_must_use)]
#![feature(auto_traits)]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(arbitrary_self_types)]
#![feature(hash_raw_entry)]

pub mod field;
mod hash_map;
// mod id;
pub mod database;
pub mod file;
mod hash_set;
pub mod id;
mod json;
mod prim;
pub mod stream_deserialize;
pub mod stream_deserializer;
pub mod stream_serialize;
pub mod stream_serializer;
mod tack;
#[cfg(test)]
mod test;
mod tester;
mod seed;
