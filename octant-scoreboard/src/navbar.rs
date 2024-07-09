use std::{cell::Cell, collections::HashMap, rc::Rc, sync::Arc};

use marshal_pointer::Rcf;

use octant_error::{octant_error, OctantResult};
use octant_runtime_server::reexports::marshal_pointer::RcfRef;
use octant_server::{PathHandler, UrlPart};
use octant_web_sys_server::{
    document::RcDocument,
    global::Global,
    html_div_element::RcHtmlDivElement,
    html_u_list_element::RcHtmlUListElement,
    node::{Node, RcNode},
};

type Content = Box<dyn Fn() -> Arc<dyn PathHandler>>;
pub struct Navbar {
    global: Rc<Global>,
    document: RcDocument,
    node: RcHtmlDivElement,
    top: RcHtmlUListElement,
    callbacks: HashMap<String, Content>,
    child: Cell<Option<RcNode>>,
}

impl Navbar {
    pub fn new(global: Rc<Global>) -> Self {
        let document = global.window().document();
        let node = document.create_div_element();
        let top = document.create_u_list_element();
        top.style().set_property("list-style-type", "none");
        top.style().set_property("margin", "0");
        top.style().set_property("padding", "0");
        top.style().set_property("overflow", "hidden");
        top.style().set_property("background-color", "#DDD");
        node.append_child(top.clone());
        Navbar {
            global: global.clone(),
            document: document.strong(),
            node,
            top,
            callbacks: HashMap::new(),
            child: Cell::new(None),
        }
    }
    pub fn register(&mut self, name: &str, title: &str, url: &str, content: Content) {
        let d = &self.document;
        let text = d.create_text_node(name.to_owned());
        let anchor = d.create_anchor_element();
        anchor.append_child(text);
        anchor.set_href(url.to_owned());
        anchor.set_push_state_handler(self.global.window().history().strong());
        anchor.style().set_property("display", "block");
        anchor.style().set_property("text-align", "center");
        anchor.style().set_property("padding", "14px 16px");
        anchor.style().set_property("text-decoration", "none");

        let li = d.create_li_element();
        li.append_child(anchor);
        li.style().set_property("float", "left");

        self.top.append_child(li.clone());
        let global = Rc::downgrade(&self.global);
        let title = title.to_owned();
        self.callbacks.insert(url.to_owned(), content);
    }
    pub fn node(&self) -> &RcfRef<dyn Node> {
        &*self.node
    }
}

impl PathHandler for Navbar {
    fn node(self: Arc<Self>) -> Rcf<dyn Node> {
        self.node.clone()
    }

    fn handle_path(self: Arc<Self>, path: UrlPart) -> OctantResult<()> {
        let (part, rest) = path.pop();
        let cb = self
            .callbacks
            .get(part)
            .ok_or_else(|| octant_error!("not found"))?;
        let handler = (cb)();
        if let Some(old) = self.child.take() {
            self.node.remove_child(old);
        }
        let new = handler.clone().node().strong();
        self.child.set(Some(new.clone()));
        self.node.append_child(new);
        handler.handle_path(rest)?;
        Ok(())
    }
}
