use std::{rc::Rc, sync::Arc};

use marshal_pointer::Rcf;
use parking_lot::Mutex;

use octant_account::SessionTable;
use octant_cookies::CookieRouter;
use octant_runtime_server::reexports::octant_error::OctantResult;
use octant_server::{session::Session, OctantApplication, PathHandler, UrlPart};
use octant_web_sys_server::{global::Global, node::Node, text::RcText};

use crate::navbar::Navbar;

pub struct ScoreApplication {
    pub cookies: Arc<CookieRouter>,
    pub sessions: Arc<SessionTable>,
    pub guesses: Mutex<Vec<Guess>>,
}

pub struct Guess {
    email: String,
    guess: String,
}

struct TextPathHandler {
    node: RcText,
}

impl TextPathHandler {
    pub fn new(global: &Rc<Global>, text: String) -> Arc<Self> {
        Arc::new(TextPathHandler {
            node: global.window().document().create_text_node(text),
        })
    }
}

impl PathHandler for TextPathHandler {
    fn node(self: Arc<Self>) -> Rcf<dyn Node> {
        self.node.clone()
    }

    fn handle_path(self: Arc<Self>, path: UrlPart) -> OctantResult<()> {
        Ok(())
    }
}

impl OctantApplication for ScoreApplication {
    fn create_path_handler(
        self: Arc<Self>,
        session: Rc<Session>,
    ) -> OctantResult<Arc<dyn PathHandler>> {
        let mut navbar = Navbar::new(session.global().clone());

        navbar.register(
            "First",
            "a title",
            "a",
            Box::new({
                let global = session.global().clone();
                move || TextPathHandler::new(&global, "a".to_owned())
            }),
        );
        navbar.register(
            "Second",
            "b title",
            "b",
            Box::new({
                let global = session.global().clone();
                move || TextPathHandler::new(&global, "b".to_owned())
            }),
        );

        Ok(Arc::new(navbar))
        // self.session.global().window().document().create_div_element();
        // Box::new(ScoreHandler { app: self })
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
