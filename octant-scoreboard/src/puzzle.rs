use marshal_pointer::{Rcf, RcfRef};
use octant_components::{Component, ComponentBuilder};
use octant_error::OctantResult;
use octant_server::session::Session;
use octant_web_sys_server::{html_div_element::RcHtmlDivElement, node::Node};
use std::rc::Rc;
use url::Url;

pub struct PuzzleComponentBuilder {
    session: Rc<Session>,
}

pub struct PuzzleComponent {
    div: RcHtmlDivElement,
}

impl PuzzleComponentBuilder {
    pub fn new(session: Rc<Session>) -> Self {
        PuzzleComponentBuilder { session }
    }
}

impl ComponentBuilder for PuzzleComponentBuilder {
    fn set_self_path(self: &RcfRef<Self>, path: &str) {}
    fn build_component(self: &RcfRef<Self>) -> OctantResult<Rcf<dyn Component>> {
        let d = self.session.global().window().document();
        let div = d.create_div_element();
        Ok(Rcf::new(PuzzleComponent { div }))
    }
}

impl Component for PuzzleComponent {
    fn node<'a>(self: &'a RcfRef<Self>) -> &'a RcfRef<dyn Node> {
        &*self.div
    }

    fn update_path(self: &RcfRef<Self>, full_path: &Url) -> OctantResult<()> {
        Ok(())
    }
}
