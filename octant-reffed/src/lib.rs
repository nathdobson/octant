#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(dispatch_from_dyn)]
#![feature(arbitrary_self_types)]
#![feature(trait_upcasting)]
mod rc;
pub use rc::*;
mod arc;
#[cfg(test)]
mod test;

pub use arc::*;
