pub mod style;

use crate::{navbar::style::NavbarStyle, Component, ComponentBuilder};
use linked_hash_map::LinkedHashMap;
use marshal_pointer::Rcf;
use octant_error::{octant_error, OctantResult};
use octant_runtime_server::reexports::marshal_pointer::RcfRef;
use octant_web_sys_server::{
    global::Global, html_anchor_element::RcHtmlAnchorElement, html_div_element::RcHtmlDivElement,
    html_li_element::RcHtmlLiElement, html_u_list_element::RcHtmlUListElement, node::Node,
};
use std::{
    collections::HashMap,
    rc::Rc,
};
use std::cell::RefCell;
use safe_once::cell::OnceCell;
use url::Url;

struct NavbarChildBuilder {
    name: String,
    builder: Rcf<dyn ComponentBuilder>,
}

pub struct NavbarBuilder {
    self_path: OnceCell<String>,
    global: Rc<Global>,
    style: Rc<NavbarStyle>,
    children: LinkedHashMap<String, NavbarChildBuilder>,
}

struct NavbarChild {
    li: RcHtmlLiElement,
    anchor: RcHtmlAnchorElement,
    builder: Rcf<dyn ComponentBuilder>,
}

pub struct Navbar {
    self_path: String,
    node: RcHtmlDivElement,
    top: RcHtmlUListElement,
    children: HashMap<String, NavbarChild>,
    state: RefCell<Option<NavbarState>>,
}

struct NavbarState {
    name: String,
    component: Rcf<dyn Component>,
}

impl NavbarBuilder {
    pub fn new(global: Rc<Global>, style: Rc<NavbarStyle>) -> Self {
        NavbarBuilder {
            self_path: OnceCell::new(),
            global,
            style,
            children: LinkedHashMap::new(),
        }
    }
    pub fn register(
        &mut self,
        name: &str,
        _title: &str,
        url: &str,
        builder: Rcf<dyn ComponentBuilder>,
    ) {
        self.children.insert(
            url.to_owned(),
            NavbarChildBuilder {
                name: name.to_string(),
                builder,
            },
        );
    }
}

impl ComponentBuilder for NavbarBuilder {
    fn set_self_path(self: &RcfRef<Self>, path: &str) {
        self.self_path.set(path.to_owned()).ok().unwrap();
        for (part, child) in self.children.iter() {
            child.builder.set_self_path(&format!("{}/{}", path, part));
        }
    }
    fn build_component(self: &RcfRef<Self>) -> OctantResult<Rcf<dyn Component>> {
        let d = self.global.window().document();
        let node = d.create_div_element();
        let top = d.create_u_list_element();
        top.class_list().add(self.style.name());
        node.append_child(top.clone());
        let mut children = HashMap::new();
        for (key, child) in self.children.iter() {
            let text = d.create_text_node(child.name.to_owned());
            let anchor = d.create_anchor_element();
            anchor.append_child(text);
            anchor.set_push_state_handler(self.global.window().history().strong());
            anchor.set_href(format!("{}/{}", self.self_path.get().unwrap(), key));

            let li = d.create_li_element();
            li.append_child(anchor.clone());

            top.append_child(li.clone());
            children.insert(
                key.clone(),
                NavbarChild {
                    li,
                    anchor,
                    builder: child.builder.clone(),
                },
            );
        }
        Ok(Rcf::new(Navbar {
            self_path: self.self_path.get().unwrap().clone(),
            node,
            top,
            children,
            state: RefCell::new(None),
        }))
    }
}

impl Component for Navbar {
    fn node<'a>(self: &'a RcfRef<Self>) -> &'a RcfRef<dyn Node> {
        &*self.node
    }

    fn update_path(self: &RcfRef<Self>, url: &Url) -> OctantResult<()> {
        let suffix = url
            .path()
            .strip_prefix(&self.self_path)
            .ok_or_else(|| octant_error!("unexpected path for navbar"))?;
        let mut part = "";
        if let Some(suffix) = suffix.strip_prefix("/") {
            if let Some((p, _)) = suffix.split_once("/") {
                part = p;
            } else {
                part = suffix;
            }
        }
        let ref mut state = *self.state.borrow_mut();
        if let Some(some_state) = state {
            if some_state.name != part {
                self.node.remove_child(some_state.component.node().strong());
                self.children
                    .get(&some_state.name)
                    .unwrap()
                    .li
                    .class_list()
                    .remove("selected");
                *state = None;
            }
        }
        if state.is_none() {
            if let Some(entry) = self.children.get(part) {
                let component = entry.builder.build_component()?;
                self.node.append_child(component.node().strong());
                entry.li.class_list().add("selected");
                *state = Some(NavbarState {
                    name: part.to_string(),
                    component,
                });
            }
        }
        if let Some(state) = state {
            state.component.update_path(url)?;
        }
        Ok(())
    }
}
