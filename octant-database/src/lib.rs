#![feature(try_blocks)]
#![deny(unused_must_use)]
#![feature(never_type)]
#![feature(unsize)]

pub mod file;
pub mod table;
pub mod database;
mod lock;
mod dirty;