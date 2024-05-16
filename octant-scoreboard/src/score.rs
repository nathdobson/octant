use std::sync::Arc;

use anyhow::anyhow;
use parking_lot::Mutex;
use url::Url;

use octant_account::SessionTable;
use octant_gui::{
    builder::{ElementExt, HtmlFormElementExt},
    element::ArcElement,
    event_loop::Page,
};
use octant_server::{cookies::CookieRouter, session::Session, Handler};
use octant_web_sys_server::prompt;

pub struct ScoreHandler {
    pub cookie_router: Arc<CookieRouter>,
    pub session_table: Arc<SessionTable>,
    pub guesses: Mutex<Vec<Guess>>,
}

pub struct Guess {
    email: String,
    guess: String,
}

impl ScoreHandler {
    pub fn handle_form(&self, session: &Arc<Session>, guess: &str) -> anyhow::Result<()> {
        let login = self
            .session_table
            .get(session)
            .ok_or_else(|| anyhow!("not logged in"))?;
        self.guesses.lock().push(Guess {
            email: login.email.clone(),
            guess: guess.to_string(),
        });
        Ok(())
    }
    pub async fn handle_impl(
        self: &Arc<Self>,
        page: ArcElement,
        session: Arc<Session>,
    ) -> anyhow::Result<()> {
        prompt(
            session.global().runtime(),
            &session.global().window(),
            "hi".to_string(),
        );
        let d = session.global().window().document();
        let input = d.create_input_element().attr("type", "text");
        let form = d
            .create_form_element()
            .child(input.clone())
            .child(d.create_element("br"))
            .child(d.create_input_element().attr("type", "submit"))
            .handler({
                let this = self.clone();
                let session = session.clone();
                move || {
                    this.handle_form(&session, &*input.input_value()).unwrap();
                }
            });
        for guess in &*self.guesses.lock() {
            page.append_child(
                d.create_element("p")
                    .child(d.create_text_node(&format!("{}: {}", guess.email, guess.guess))),
            );
        }
        page.child(form);
        self.cookie_router.update(&session).await?;
        let user = self.session_table.get(&session);
        log::info!("verified user = {:?}", user);
        Ok(())
    }
}

impl Handler for ScoreHandler {
    fn prefix(&self) -> String {
        "score".to_string()
    }

    fn handle(self: Arc<Self>, url: &Url, session: Arc<Session>) -> anyhow::Result<Page> {
        let d = session.global().window().document();
        let page = d.create_element("div");

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
