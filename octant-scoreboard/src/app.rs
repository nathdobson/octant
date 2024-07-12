use crate::puzzle::PuzzleComponentBuilder;
use marshal_pointer::{Rcf, RcfRef};
use octant_account::{
    login::LoginComponentBuilder, register::RegisterComponentBuilder, style::AccountStyle,
    SessionTable,
};
use octant_components::{
    css_scope::CssScopeSet,
    navbar::{style::NavbarStyle, NavbarBuilder},
    Component, ComponentBuilder,
};
use octant_cookies::CookieRouter;
use octant_database::database::ArcDatabase;
use octant_runtime_server::reexports::octant_error::OctantResult;
use octant_server::{session::Session, OctantApplication};
use octant_web_sys_server::{global::Global, node::Node, text::RcText};
use parking_lot::Mutex;
use safe_once::cell::OnceCell;
use std::{rc::Rc, sync::Arc};
use url::Url;

pub struct ScoreApplication {
    pub db: ArcDatabase,
    pub cookies: Arc<CookieRouter>,
    pub sessions: Arc<SessionTable>,
    pub guesses: Mutex<Vec<Guess>>,
}

pub struct Guess {
    email: String,
    guess: String,
}

struct TextComponentBuilder {
    self_path: OnceCell<String>,
    global: Rc<Global>,
    userdata: String,
}

struct TextComponent {
    userdata: String,
    self_path: String,
    node: RcText,
}

impl TextComponentBuilder {
    pub fn new(global: Rc<Global>, userdata: String) -> Rcf<Self> {
        Rcf::new(TextComponentBuilder {
            self_path: OnceCell::new(),
            global,
            userdata,
        })
    }
}

impl ComponentBuilder for TextComponentBuilder {
    fn set_self_path(self: &RcfRef<Self>, path: &str) {
        self.self_path.set(path.to_owned()).ok().unwrap();
    }

    fn build_component(self: &RcfRef<Self>) -> OctantResult<Rcf<dyn Component>> {
        Ok(Rcf::new(TextComponent {
            userdata: self.userdata.clone(),
            self_path: self.self_path.get().unwrap().clone(),
            node: self
                .global
                .window()
                .document()
                .create_text_node("uninit".to_string()),
        }))
    }
}

impl Component for TextComponent {
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
    fn create_component_builder(
        self: Arc<Self>,
        session: Rc<Session>,
    ) -> OctantResult<Rcf<dyn ComponentBuilder>> {
        let global = session.global();
        let mut scopes = CssScopeSet::new(global.clone());
        let navbar_style = Rc::new(NavbarStyle::new(&mut scopes));
        let account_style = Rc::new(AccountStyle::new(&mut scopes));

        let mut navbar = NavbarBuilder::new(global.clone(), navbar_style.clone());

        navbar.register(
            "Puzzle 1",
            "",
            "puzzle1",
            PuzzleComponentBuilder::new(session.clone()),
        );
        navbar.register(
            "Second",
            "b title",
            "b",
            TextComponentBuilder::new(global.clone(), "b".to_owned()),
        );
        navbar.register(
            "Third",
            "c title",
            "c",
            TextComponentBuilder::new(global.clone(), "c".to_owned()),
        );
        navbar.register(
            "Login",
            "Login",
            "login",
            Rcf::new(LoginComponentBuilder::new(
                self.db.clone(),
                self.cookies.clone(),
                self.sessions.clone(),
                session.clone(),
                account_style.clone(),
            )),
        );
        navbar.register(
            "Register",
            "Register",
            "register",
            Rcf::new(RegisterComponentBuilder::new(
                self.db.clone(),
                session,
                account_style.clone(),
            )),
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
