use marshal_pointer::Rcf;
use octant_error::{octant_error, OctantResult};
use octant_runtime_server::reexports::marshal_pointer::RcfRef;
use octant_server::{PathHandler, UrlPart};
use octant_web_sys_server::{
    document::RcDocument,
    global::Global,
    html_div_element::RcHtmlDivElement,
    node::{Node, RcNode},
};
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    ops::Deref,
    rc::Rc,
    sync::Arc,
};

type Content = Box<dyn Fn() -> Arc<dyn PathHandler>>;
pub struct Navbar {
    global: Rc<Global>,
    document: RcDocument,
    node: RcHtmlDivElement,
    top: RcHtmlDivElement,
    callbacks: HashMap<String, Content>,
    child: Cell<Option<RcNode>>,
}

impl Navbar {
    pub fn new(global: Rc<Global>) -> Self {
        let document = global.window().document();
        let node = document.create_div_element();
        let top = document.create_div_element();
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
        let anchor = self.document.create_anchor_element();
        anchor.set_href(url.to_owned());
        let text = self.document.create_text_node(name.to_owned());
        anchor.append_child(text);
        self.top.append_child(anchor.clone());
        let global = Rc::downgrade(&self.global);
        let title = title.to_owned();
        anchor.set_push_state_handler(self.global.window().history().strong());
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
        let mut handler = (cb)();
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
