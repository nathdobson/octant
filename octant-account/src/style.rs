use octant_components::css_scope::{CssScope, CssScopeSet};

pub struct AccountStyle(CssScope);

impl AccountStyle {
    pub fn new(scopes: &mut CssScopeSet) -> Self {
        AccountStyle(scopes.add(
            "account",
            None,
            r##"
            :scope {
                padding: 10px;
            }
            * {
                box-sizing: border-box
            }
            input[type=text] {
                width: 100%;
                padding: 10px;
                margin: 5px 0 5px 0;
                display: inline-block;
                border: none;
                background: #EEE;
            }
            input[type=text]:focus {
                background-color: #DDD;
                outline: none;
            }
            hr {
                border: 1px solid #EEE;
            }
            input[type=submit] {
                background-color: #080;
                color: white;
                padding: 10px;
                margin: 5px 0 5px 0;
                border: none;
                cursor: pointer;
                width: 100%;
            }
            input[type="submit"]:hover {
                background-color: #0A0;
            }

        "##,
        ))
    }
    pub fn name(&self) -> &str {
        self.0.name()
    }
}
