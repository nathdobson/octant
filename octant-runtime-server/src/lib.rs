#![feature(unsize)]
#![feature(macro_metavar_expr)]
#![allow(unused_variables)]
#![deny(unused_must_use)]

pub mod handle;
pub mod define_sys_rpc;
pub mod define_sys_class;
#[doc(hidden)]
pub mod reexports {
    pub use anyhow;
    pub use catalog;
    pub use paste;
    pub use serde;

    pub use octant_object;
    pub use octant_serde;
}

#[cfg_attr(side="client",path="client_runtime.rs")]
#[cfg_attr(side="server",path="server_runtime.rs")]
pub mod runtime;

#[cfg_attr(side="client",path="client_peer.rs")]
#[cfg_attr(side="server",path="server_peer.rs")]
pub mod peer;
pub mod proto;
