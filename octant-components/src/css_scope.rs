use octant_web_sys_server::{global::Global, html_style_element::RcHtmlStyleElement};
use std::{collections::HashMap, rc::Rc};

pub struct CssScopeSet {
    global: Rc<Global>,
    names: HashMap<String, usize>,
    style: RcHtmlStyleElement,
}

pub struct CssScope {
    name: String,
}

impl CssScopeSet {
    pub fn new(global: Rc<Global>) -> Self {
        let style = global.window().document().create_style_element();
        global
            .window()
            .document()
            .head()
            .append_child(style.clone());
        CssScopeSet {
            global,
            names: HashMap::new(),
            style,
        }
    }
    pub fn add(&mut self, debug: &str, lower: Option<&str>, content: &str) -> CssScope {
        let index = self.names.entry(debug.to_string()).or_default();
        let name = format!("{}_{}", debug, index);
        *index += 1;
        if let Some(lower) = lower {
            self.style
                .sheet()
                .insert_rule(format!("@scope (.{name:}) to ({lower:}) {{{content:}}}"));
        } else {
            self.style
                .sheet()
                .insert_rule(format!("@scope (.{name:}) {{{content:}}}"));
        }
        CssScope { name }
    }
}

impl CssScope {
    pub fn name(&self) -> &str {
        &self.name
    }
}
