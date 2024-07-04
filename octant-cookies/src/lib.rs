use cookie::Cookie;
use parking_lot::Mutex;
use std::{
    collections::HashMap,
    future::Future,
    rc::Rc,
    sync::{Arc, Weak},
};
use uuid::Uuid;
use warp::Filter;
use weak_table::WeakValueHashMap;

use itertools::Itertools;
use octant_runtime_server::reexports::octant_error::OctantResult;
use octant_server::{
    session::{Session, SessionData},
    IntoWarpHandler, OctantServer, WarpHandler,
};

pub struct CookieRouter {
    create_cookies: Mutex<HashMap<Uuid, String>>,
    update_cookies: Mutex<WeakValueHashMap<Uuid, Weak<SharedCookieData>>>,
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
struct SharedCookieData {
    cookies: Mutex<HashMap<String, Arc<String>>>,
}

#[derive(Default, Debug)]
pub struct CookieData {
    shared_cookies: Arc<SharedCookieData>,
}

impl SessionData for CookieData {}

impl CookieData {
    pub fn get(&self, key: &str) -> Option<Arc<String>> {
        self.shared_cookies.cookies.lock().get(key).cloned()
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
    pub fn update_start(&self, session: &Rc<Session>) -> (Uuid, CookieUpdateGuard) {
        let update_token = Uuid::new_v4();
        self.update_cookies.lock().insert(
            update_token,
            session.data::<CookieData>().shared_cookies.clone(),
        );
        (
            update_token,
            CookieUpdateGuard {
                token: Default::default(),
                router: &self,
            },
        )
    }
    pub fn update_finish(&self, token: Uuid, cookies: HashMap<String, Arc<String>>) {
        if let Some(data) = self.update_cookies.lock().get(&token) {
            *data.cookies.lock() = cookies;
        }
    }
    pub fn create<'a>(
        &'a self,
        session: &'a Rc<Session>,
        cookie: String,
    ) -> impl 'a + Future<Output = OctantResult<()>> {
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
    pub async fn update(&self, session: &Rc<Session>) -> OctantResult<()> {
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

    pub fn create_cookie_filter(self: &Arc<Self>) -> WarpHandler {
        warp::path("create_cookie")
            .and(warp::query::<HashMap<String, String>>())
            .map({
                let this = self.clone();
                move |q: HashMap<String, String>| {
                    let token: Uuid = q.get("token").unwrap().parse().unwrap();
                    let cookie = this.create_finish(token).unwrap();
                    let res = warp::reply::json(&());
                    let res = warp::reply::with_header(res, "set-cookie", format!("{}", cookie));
                    res
                }
            })
            .into_warp_handler()
    }
    pub fn update_cookie_filter(self: &Arc<Self>) -> WarpHandler {
        warp::path("update_cookie")
            .and(warp::query::<HashMap<String, String>>())
            .and(warp::header("Cookie"))
            .map({
                let this = self.clone();
                move |q: HashMap<String, String>, cookie: String| {
                    let cookies = Cookie::split_parse(&cookie)
                        .map_ok(|x| (x.name().to_string(), Arc::new(x.value().to_string())))
                        .collect::<Result<HashMap<_, _>, _>>()
                        .unwrap();
                    let token: Uuid = q.get("token").unwrap().parse().unwrap();
                    this.update_finish(token, cookies);
                    let res = warp::reply::json(&());
                    res
                }
            })
            .into_warp_handler()
    }
    pub fn register(self: &Arc<Self>, server: &mut OctantServer) {
        server.add_warp_handler(self.create_cookie_filter());
        server.add_warp_handler(self.update_cookie_filter());
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
