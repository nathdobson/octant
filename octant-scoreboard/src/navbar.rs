use marshal_pointer::Rcf;
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use crate::css_scope::{CssScope, CssScopeSet};
use octant_error::OctantResult;
use octant_runtime_server::reexports::marshal_pointer::RcfRef;
use octant_server::{PathHandler, UrlPart};
use octant_web_sys_server::{
    document::RcDocument, global::Global, html_div_element::RcHtmlDivElement,
    html_li_element::RcHtmlLiElement, html_u_list_element::RcHtmlUListElement, node::Node,
};

struct ChildEntry {
    li: RcHtmlLiElement,
    content: Content,
    current_content: RefCell<Option<Arc<dyn PathHandler>>>,
}
type Content = Box<dyn Fn() -> Arc<dyn PathHandler>>;
pub struct Navbar {
    global: Rc<Global>,
    document: RcDocument,
    node: RcHtmlDivElement,
    top: RcHtmlUListElement,
    children: HashMap<String, ChildEntry>,
    current_child: RefCell<Option<String>>,
}

pub struct NavbarStyle(CssScope);

impl NavbarStyle {
    pub fn new(set: &mut CssScopeSet) -> Self {
        NavbarStyle(set.add(
            "navbar",
            None,
            r##"
                :scope {
                    list-style-type: none;
                    margin: 0;
                    padding: 0;
                    overflow: hidden;
                    background-color: #BBF;
                }
                a {
                    display: block;
                    text-decoration: none;
                    text-align: center;
                    padding: 14px 16px;
                }
                li {
                    float: left;
                    background-color: #BBF;
                }
                li:hover {
                    background-color: #BFB;
                }
                li.selected {
                    background-color: #FBB;
                }
        "##,
        ))
    }
}

impl Navbar {
    pub fn new(global: Rc<Global>, style: &NavbarStyle) -> Self {
        let document = global.window().document();
        let node = document.create_div_element();
        let top = document.create_u_list_element();
        top.class_list().add(style.0.name());
        node.append_child(top.clone());
        Navbar {
            global: global.clone(),
            document: document.strong(),
            node,
            top,
            children: HashMap::new(),
            current_child: RefCell::new(None),
        }
    }
    pub fn register(&mut self, name: &str, title: &str, url: &str, content: Content) {
        let d = &self.document;
        let text = d.create_text_node(name.to_owned());
        let anchor = d.create_anchor_element();
        anchor.append_child(text);
        anchor.set_href(url.to_owned());
        anchor.set_push_state_handler(self.global.window().history().strong());

        let li = d.create_li_element();
        li.append_child(anchor);

        self.top.append_child(li.clone());
        let global = Rc::downgrade(&self.global);
        let title = title.to_owned();
        self.children.insert(
            url.to_owned(),
            ChildEntry {
                li,
                content,
                current_content: RefCell::new(None),
            },
        );
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
        let ref mut current_child = *self.current_child.borrow_mut();
        if current_child.as_deref() != Some(part) {
            if let Some(current_child) = current_child {
                if let Some(entry) = self.children.get(current_child) {
                    if let Some(content) = entry.current_content.borrow_mut().take() {
                        self.node.remove_child(content.node());
                    }
                    entry.li.class_list().remove("selected");
                }
            }
            *current_child = Some(part.to_owned());
            if let Some(entry) = self.children.get(part) {
                let content = (entry.content)();
                *entry.current_content.borrow_mut() = Some(content.clone());
                self.node.append_child(content.node());
                entry.li.class_list().add("selected");
            }
        }
        if let Some(entry) = self.children.get(part) {
            entry
                .current_content
                .borrow()
                .as_ref()
                .unwrap()
                .clone()
                .handle_path(rest)?;
        }

        Ok(())
    }
}
