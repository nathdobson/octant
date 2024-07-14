use marshal_pointer::{EmptyRcf, Rcf, RcfRef};
use octant_components::{Component, ComponentBuilder};
use octant_database::database::ArcDatabase;
use octant_error::OctantResult;
use octant_server::session::Session;
use octant_web_sys_server::{
    attributes::input_type::InputType, html_div_element::RcHtmlDivElement,
    html_input_element::RcHtmlInputElement, node::Node,
};
use std::rc::Rc;
use url::Url;

pub struct PuzzleComponentBuilder {
    db: ArcDatabase,
    session: Rc<Session>,
}

pub struct PuzzleComponent {
    session: Rc<Session>,
    div: RcHtmlDivElement,
    guess: RcHtmlInputElement,
}

impl PuzzleComponentBuilder {
    pub fn new(db: ArcDatabase, session: Rc<Session>) -> Rcf<Self> {
        Rcf::new(PuzzleComponentBuilder { db, session })
    }
}

impl ComponentBuilder for PuzzleComponentBuilder {
    fn set_self_path(self: &RcfRef<Self>, path: &str) {}
    fn build_component(self: &RcfRef<Self>) -> OctantResult<Rcf<dyn Component>> {
        let this = EmptyRcf::<PuzzleComponent>::new();
        let d = self.session.global().window().document();
        let div = d.create_div_element();
        let guess;
        div.append_child({
            let form = d.create_form_element();
            form.append_child({
                guess = d.create_input_element();
                guess.set_type(InputType::Text);
                guess.set_placeholder("Enter Guess".to_string());
                guess.clone()
            });
            form.append_child({
                let button = d.create_input_element();
                button.set_type(InputType::Button);
                button.set_value("Submit Guess".to_string());
                button
            });
            form.set_form_submit_handler(Box::new({
                let this = this.downgrade();
                move |()| {
                    if let Some(this) = this.upgrade() {
                        this.submit_guess();
                    }
                    Ok(())
                }
            }));
            form
        });
        let content_div;
        div.append_child({
            content_div = d.create_div_element();
            content_div.clone()
        });
        self.session.global().runtime().spawner().spawn({
            let this = this.downgrade();
            async move {
                if let Some(this) = this.upgrade() {
                    let global = this.session.global();
                    let request_init = global.new_request_init();
                    let request = this.session.global().new_request(
                        "/static/octant-scoreboard/puzzle1.htmli".to_owned(),
                        request_init,
                    );
                    let content = this.session.global().window().fetch(request).await?;
                    let content = content.remote_text().await?.strong();
                    content_div.set_inner_html(content);
                }
                Ok(())
            }
        });

        Ok(this.into_strong(PuzzleComponent {
            session: self.session.clone(),
            div,
            guess,
        }))
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

impl PuzzleComponent {
    fn submit_guess(self: &RcfRef<Self>) {}
}
