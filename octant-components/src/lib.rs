#![feature(trait_upcasting)]
#![feature(arbitrary_self_types)]
#![deny(unused_must_use)]
#![allow(dead_code)]

use marshal_pointer::{Rcf, RcfRef};
use octant_runtime_server::reexports::octant_error::OctantResult;
use octant_web_sys_server::node::Node;
use url::Url;

pub mod css_scope;
pub mod navbar;

pub trait ComponentBuilder {
    fn set_self_path(self: &RcfRef<Self>, path: &str);
    fn build_component(self: &RcfRef<Self>) -> OctantResult<Rcf<dyn Component>>;
}

pub trait Component {
    fn node<'a>(self: &'a RcfRef<Self>) -> &'a RcfRef<dyn Node>;
    fn update_path(self: &RcfRef<Self>, full_path: &Url) -> OctantResult<()>;
}
