use octant_runtime_server::reexports::marshal_pointer::RcfRef;
use octant_web_sys_server::{
    document::RcDocument,
    global::Global,
    html_div_element::{RcHtmlDivElement},
    node::{Node, RcNode},
};
use std::rc::Rc;

pub struct Navbar {
    document: RcDocument,
    node: RcHtmlDivElement,
    top: RcHtmlDivElement,
}

impl Navbar {
    pub fn new(global: Rc<Global>) -> Self {
        let document = global.window().document();
        let node = document.create_div_element();
        let top = document.create_div_element();
        node.append_child(top.clone());
        Navbar {
            document: document.strong(),
            node,
            top,
        }
    }
    pub fn register<F: Fn() -> RcNode>(&mut self, name: &str, f: F) {
        let anchor = self.document.create_anchor_element();
        anchor.set_href(name.to_owned());
        let text = self.document.create_text_node(name.to_owned());
        anchor.append_child(text);
        self.top.append_child(anchor);
    }
    pub fn node(&self) -> &RcfRef<dyn Node> {
        &*self.node
    }
}
