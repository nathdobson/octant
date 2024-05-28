use std::{
    collections::HashMap,
    future::Future,
    sync::{Arc, Weak},
};

use parking_lot::Mutex;
use uuid::Uuid;
use weak_table::WeakValueHashMap;

use crate::session::{Session, SessionData};

pub struct CookieRouter {
    create_cookies: Mutex<HashMap<Uuid, String>>,
    update_cookies: Mutex<WeakValueHashMap<Uuid, Weak<Session>>>,
}

pub struct CookieCreateGuard<'a> {
    token: Uuid,
    router: &'a CookieRouter,
}

pub struct CookieUpdateGuard<'a> {
    token: Uuid,
    router: &'a CookieRouter,
}

#[derive(Default, Debug)]
pub struct CookieData {
    cookies: Mutex<HashMap<String, Arc<String>>>,
}

impl SessionData for CookieData {}

impl CookieData {
    pub fn get(&self, key: &str) -> Option<Arc<String>> {
        self.cookies.lock().get(key).cloned()
    }
}

impl CookieRouter {
    pub fn new() -> Arc<CookieRouter> {
        Arc::new(CookieRouter {
            create_cookies: Mutex::new(HashMap::new()),
            update_cookies: Mutex::new(WeakValueHashMap::new()),
        })
    }
    pub fn create_start(&self, value: String) -> (Uuid, CookieCreateGuard) {
        let token = Uuid::new_v4();
        self.create_cookies.lock().insert(token, value);
        (
            token,
            CookieCreateGuard {
                token,
                router: self,
            },
        )
    }
    pub fn create_finish(&self, token: Uuid) -> Option<String> {
        self.create_cookies.lock().get(&token).cloned()
    }
    pub fn update_start(&self, session: &Arc<Session>) -> (Uuid, CookieUpdateGuard) {
        let update_token = Uuid::new_v4();
        self.update_cookies
            .lock()
            .insert(update_token, session.clone());
        (
            update_token,
            CookieUpdateGuard {
                token: Default::default(),
                router: &self,
            },
        )
    }
    pub fn update_finish(&self, token: Uuid, cookies: HashMap<String, Arc<String>>) {
        if let Some(session) = self.update_cookies.lock().get(&token) {
            *session.data::<CookieData>().cookies.lock() = cookies;
        }
    }
    pub fn create<'a>(
        &'a self,
        session: &'a Arc<Session>,
        cookie: String,
    ) -> impl 'a + Send + Future<Output = anyhow::Result<()>> {
        async move {
            let (cookie_token, _guard) = self.create_start(cookie);
            let request_init = session.global().new_request_init();
            let request = session.global().new_request(
                format!("/create_cookie?token={}", cookie_token),
                request_init,
            );
            session
                .global()
                .window()
                .fetch(request)
                .await?
                .text()
                .await?;
            Ok(())
        }
    }
    pub async fn update(&self, session: &Arc<Session>) -> anyhow::Result<()> {
        let (cookie_token, _guard) = self.update_start(&session);
        let request_init = session.global().new_request_init();
        let request = session.global().new_request(
            format!("/update_cookie?token={}", cookie_token),
            request_init,
        );
        session
            .global()
            .window()
            .fetch(request)
            .await?
            .text()
            .await?;
        log::info!("Cookies: {:?}", session.data::<CookieData>());
        Ok(())
    }
}

impl<'a> Drop for CookieCreateGuard<'a> {
    fn drop(&mut self) {
        self.router.create_cookies.lock().remove(&self.token);
    }
}

impl<'a> Drop for CookieUpdateGuard<'a> {
    fn drop(&mut self) {
        self.router.update_cookies.lock().remove(&self.token);
    }
}
