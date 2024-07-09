use crate::css_scope::{CssScope, CssScopeSet};

pub struct NavbarStyle(CssScope);

impl NavbarStyle {
    pub fn name(&self) -> &str {
        self.0.name()
    }
    pub fn new(set: &mut CssScopeSet) -> Self {
        NavbarStyle(set.add(
            "navbar",
            None,
            r##"
                :scope {
                    list-style-type: none;
                    margin: 3px;
                    padding: 0;
                    overflow: hidden;
                    background-color: #000;
                }
                a {
                    display: block;
                    text-decoration: none;
                    text-align: center;
                    padding: 14px 16px;
                    color: white;
                }
                li {
                    float: left;
                    background-color: #444;
                    margin-top: 3px;
                    margin-bottom: 3px;
                    margin-left: 3px;
                }
                li:hover {
                    background-color: #555;
                }
                li.selected {
                    background-color: #888;
                }
            "##,
        ))
    }
}
