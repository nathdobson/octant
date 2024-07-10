#![feature(trait_upcasting)]
#![feature(arbitrary_self_types)]
#![deny(unused_must_use)]
#![allow(dead_code)]

use marshal_pointer::{Rcf, RcfRef};
use octant_runtime_server::reexports::octant_error::OctantResult;
use octant_web_sys_server::node::Node;
use url::Url;

pub mod navbar;
pub mod css_scope;

pub trait PathComponentBuilder {
    fn build(self: &RcfRef<Self>, self_path: &str) -> OctantResult<Rcf<dyn PathComponent>>;
}

pub trait PathComponent {
    fn node<'a>(self: &'a RcfRef<Self>) -> &'a RcfRef<dyn Node>;
    fn update_path(self: &RcfRef<Self>, full_path: &Url) -> OctantResult<()>;
}

