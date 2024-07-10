use std::{rc::Rc, sync::Arc};

use marshal_pointer::{Rcf, RcfRef};
use octant_account::SessionTable;
use octant_components::{
    css_scope::CssScopeSet,
    navbar::{style::NavbarStyle, NavbarBuilder},
    PathComponent, PathComponentBuilder,
};
use octant_cookies::CookieRouter;
use octant_runtime_server::reexports::octant_error::OctantResult;
use octant_server::{session::Session, OctantApplication};
use octant_web_sys_server::{global::Global, node::Node, text::RcText};
use parking_lot::Mutex;
use url::Url;

pub struct ScoreApplication {
    pub cookies: Arc<CookieRouter>,
    pub sessions: Arc<SessionTable>,
    pub guesses: Mutex<Vec<Guess>>,
}

pub struct Guess {
    email: String,
    guess: String,
}

struct TextPathComponentBuilder {
    global: Rc<Global>,
    userdata: String,
}

struct TextPathComponent {
    userdata: String,
    self_path: String,
    node: RcText,
}

impl TextPathComponentBuilder {
    pub fn new(global: Rc<Global>, userdata: String) -> Rcf<Self> {
        Rcf::new(TextPathComponentBuilder { global, userdata })
    }
}

impl PathComponentBuilder for TextPathComponentBuilder {
    fn build(self: &RcfRef<Self>, self_path: &str) -> OctantResult<Rcf<dyn PathComponent>> {
        Ok(Rcf::new(TextPathComponent {
            userdata: self.userdata.clone(),
            self_path: self_path.to_string(),
            node: self
                .global
                .window()
                .document()
                .create_text_node("uninit".to_string()),
        }))
    }
}

impl PathComponent for TextPathComponent {
    fn node<'a>(self: &'a RcfRef<Self>) -> &'a RcfRef<dyn Node> {
        &*self.node
    }

    fn update_path(self: &RcfRef<Self>, path: &Url) -> OctantResult<()> {
        self.node
            .set_node_value(format!("{} {} {}", self.userdata, self.self_path, path));
        Ok(())
    }
}

impl OctantApplication for ScoreApplication {
    fn create_path_component_builder(
        self: Arc<Self>,
        session: Rc<Session>,
    ) -> OctantResult<Rcf<dyn PathComponentBuilder>> {
        let global = session.global();
        let mut scopes = CssScopeSet::new(global.clone());
        let style = Rc::new(NavbarStyle::new(&mut scopes));
        let mut navbar = NavbarBuilder::new(global.clone(), style.clone());

        let mut child_navbar = NavbarBuilder::new(global.clone(), style.clone());
        child_navbar.register(
            "Part X",
            "ax",
            "x",
            TextPathComponentBuilder::new(session.global().clone(), "hi".to_string()),
        );

        navbar.register("First", "a title", "a", Rcf::new(child_navbar));
        navbar.register(
            "Second",
            "b title",
            "b",
            TextPathComponentBuilder::new(global.clone(), "b".to_owned()),
        );
        navbar.register(
            "Third",
            "c title",
            "c",
            TextPathComponentBuilder::new(global.clone(), "c".to_owned()),
        );

        Ok(Rcf::new(navbar))
    }
}

// impl ScoreApplication {
//     pub fn handle_form(&self, session: &Rc<Session>, guess: &str) -> OctantResult<()> {
//         let login = self
//             .sessions
//             .get(session)
//             .ok_or_else(|| octant_error!("not logged in"))?;
//         self.guesses.lock().push(Guess {
//             email: login.email.clone(),
//             guess: guess.to_string(),
//         });
//         Ok(())
//     }
//     pub async fn handle_impl(
//         self: &Arc<Self>,
//         page: RcElement,
//         session: Rc<Session>,
//     ) -> OctantResult<()> {
//         let global = session.global();
//         let history = global.window().history();
//         history.replace_state("score".to_string(), Some("/site/score/".to_string()));
//         let mut navbar = Navbar::new(session.global().clone());
//         page.append_child(navbar.node().strong());
//         navbar.register("a button", "a title", "a", || {
//             todo!();
//         });
//         navbar.register("b button", "b title", "b", || {
//             todo!();
//         });
//         // let d = session.global().window().document();
//         // let input = d.create_input_element().attr("type", "text");
//         // let form = d
//         //     .create_form_element()
//         //     .child(input.clone())
//         //     .child(d.create_element("br".to_string()))
//         //     .child(d.create_input_element().attr("type", "submit"))
//         //     .handler({
//         //         let this = self.clone();
//         //         let session = session.clone();
//         //         session.global().new_event_listener({
//         //             let session = session.clone();
//         //             move || {
//         //                 this.handle_form(&session, &*input.input_value()).unwrap();
//         //             }
//         //         })
//         //     });
//         // for guess in &*self.guesses.lock() {
//         //     page = page.child(
//         //         d.create_element("p".to_string())
//         //             .child(d.create_text_node(format!("{}: {}", guess.email, guess.guess))),
//         //     );
//         // }
//         // page.child(form);
//         // self.cookies.update(&session).await?;
//         // let user = self.sessions.get(&session);
//         // log::info!("verified user = {:?}", user);
//         Ok(())
//     }
// }
//
// pub struct ScoreHandler {
//     app: Arc<ScoreApplication>,
//     node: Arc<dyn Node>,
// }
//
// impl PathHandler for ScoreHandler {
//     fn node(&self) -> &RcfRef<dyn Node> {
//         &self.node
//     }
//
//     fn handle_path(&mut self, url: UrlPart) -> OctantResult<()> {
//         let page = ;
//         self.session.global().runtime().spawner().spawn({
//             let session = self.session.clone();
//             let page = page.clone();
//             async move {
//                 self.app.handle_impl(page, session.clone()).await?;
//                 Ok(())
//             }
//         });
//         Ok(page)
//     }
// }
