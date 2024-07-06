use std::{rc::Rc, sync::Arc};

use parking_lot::Mutex;
use url::Url;

use crate::navbar::Navbar;
use octant_account::SessionTable;
use octant_cookies::CookieRouter;
use octant_runtime_server::reexports::octant_error::{octant_error, OctantResult};
use octant_server::{session::Session, Handler, Page};
use octant_web_sys_server::element::RcElement;

pub struct ScoreHandler {
    pub cookies: Arc<CookieRouter>,
    pub sessions: Arc<SessionTable>,
    pub guesses: Mutex<Vec<Guess>>,
}

pub struct Guess {
    email: String,
    guess: String,
}

impl ScoreHandler {
    pub fn handle_form(&self, session: &Rc<Session>, guess: &str) -> OctantResult<()> {
        let login = self
            .sessions
            .get(session)
            .ok_or_else(|| octant_error!("not logged in"))?;
        self.guesses.lock().push(Guess {
            email: login.email.clone(),
            guess: guess.to_string(),
        });
        Ok(())
    }
    pub async fn handle_impl(
        self: &Arc<Self>,
        page: RcElement,
        session: Rc<Session>,
    ) -> OctantResult<()> {
        let global = session.global();
        let history = global.window().history();
        history.replace_state(&history.state(global), "/site/score/".to_string());
        let mut navbar = Navbar::new(session.global().clone());
        page.append_child(navbar.node().strong());
        navbar.register("a", || {
            todo!();
        });
        navbar.register("b", || {
            todo!();
        });
        // let d = session.global().window().document();
        // let input = d.create_input_element().attr("type", "text");
        // let form = d
        //     .create_form_element()
        //     .child(input.clone())
        //     .child(d.create_element("br".to_string()))
        //     .child(d.create_input_element().attr("type", "submit"))
        //     .handler({
        //         let this = self.clone();
        //         let session = session.clone();
        //         session.global().new_event_listener({
        //             let session = session.clone();
        //             move || {
        //                 this.handle_form(&session, &*input.input_value()).unwrap();
        //             }
        //         })
        //     });
        // for guess in &*self.guesses.lock() {
        //     page = page.child(
        //         d.create_element("p".to_string())
        //             .child(d.create_text_node(format!("{}: {}", guess.email, guess.guess))),
        //     );
        // }
        // page.child(form);
        // self.cookies.update(&session).await?;
        // let user = self.sessions.get(&session);
        // log::info!("verified user = {:?}", user);
        Ok(())
    }
}

impl Handler for ScoreHandler {
    fn prefix(&self) -> String {
        "score".to_string()
    }

    fn handle(self: Arc<Self>, url: &Url, session: Rc<Session>) -> OctantResult<Page> {
        let page = session.global().window().document().create_div_element();
        session.global().runtime().spawner().spawn({
            let session = session.clone();
            let page = page.clone();
            async move {
                self.handle_impl(page, session.clone()).await?;
                Ok(())
            }
        });
        Ok(Page::new(session.global().clone(), page))
    }
}
