use marshal_pointer::{Rcf, RcfRef};
use octant_components::{Component, ComponentBuilder};
use octant_error::OctantResult;
use octant_server::session::Session;
use octant_web_sys_server::{html_div_element::RcHtmlDivElement, node::Node};
use std::rc::Rc;
use url::Url;

pub struct PuzzleComponentBuilder {
    session: Rc<Session>,
}

pub struct PuzzleComponent {
    div: RcHtmlDivElement,
}

impl PuzzleComponentBuilder {
    pub fn new(session: Rc<Session>) -> Rcf<Self> {
        Rcf::new(PuzzleComponentBuilder { session })
    }
}

impl ComponentBuilder for PuzzleComponentBuilder {
    fn set_self_path(self: &RcfRef<Self>, path: &str) {}
    fn build_component(self: &RcfRef<Self>) -> OctantResult<Rcf<dyn Component>> {
        let d = self.session.global().window().document();
        let div = d.create_div_element();

        div.append_child({
            let form = d.create_form_element();
            form.append_child({
                let text=d.create_input_element();

                text
            });
            form
        });
        let content_div;
        div.append_child({
            content_div = d.create_div_element();
            content_div.clone()
        });
        let this = self.strong();
        self.session.global().runtime().spawner().spawn({
            let content_div = content_div.clone();
            async move {
                let global = this.session.global();
                let request_init = global.new_request_init();
                let request = this.session.global().new_request(
                    "/static/octant-scoreboard/puzzle1.htmli".to_owned(),
                    request_init,
                );
                let content = this.session.global().window().fetch(request).await?;
                let content = content.remote_text().await?.strong();
                content_div.set_inner_html(content);
                Ok(())
            }
        });

        Ok(Rcf::new(PuzzleComponent { div }))
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
