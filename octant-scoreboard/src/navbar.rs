use octant_error::{octant_error, OctantResult};
use octant_runtime_server::reexports::marshal_pointer::RcfRef;
use octant_server::{PathHandler, UrlPart};
use octant_web_sys_server::{
    document::RcDocument,
    global::Global,
    html_div_element::RcHtmlDivElement,
    node::{Node, RcNode},
};
use std::{collections::HashMap, ops::Deref, rc::Rc};
use std::sync::Arc;
use marshal_pointer::Rcf;

type Content = Box<dyn Fn() -> Arc<dyn PathHandler>>;
pub struct Navbar {
    global: Rc<Global>,
    document: RcDocument,
    node: RcHtmlDivElement,
    top: RcHtmlDivElement,
    callbacks: HashMap<String, Content>,
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
        }
    }
    pub fn register(&mut self, name: &str, title: &str, url: &str, content: Content) {
        let anchor = self.document.create_anchor_element();
        anchor.set_href(name.to_owned());
        let text = self.document.create_text_node(name.to_owned());
        anchor.append_child(text);
        self.top.append_child(anchor.clone());
        let global = Rc::downgrade(&self.global);
        let title = title.to_owned();

        let listener = self.global.new_event_listener({
            let url = url.to_owned();
            move || {
                if let Some(global) = global.upgrade() {
                    global
                        .window()
                        .history()
                        .push_state(title.clone(), Some(url.clone()));
                }
                //
            }
        });
        listener.set_prevent_default(true);
        anchor.add_listener("click", listener);
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
        self.node.append_child(handler.clone().node().strong());
        handler.handle_path(rest)?;
        Ok(())
    }
}
