#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(dispatch_from_dyn)]
#![feature(arbitrary_self_types)]
#![feature(trait_upcasting)]
pub mod arc;
pub mod rc;
#[cfg(test)]
mod test;
