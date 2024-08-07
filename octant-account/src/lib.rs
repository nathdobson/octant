#![feature(trait_upcasting)]
#![allow(dead_code)]
#![feature(arbitrary_self_types)]
#![allow(unused_variables)]
#![deny(unused_must_use)]

use std::{collections::HashMap, rc::Rc, sync::Arc};

use base64urlsafedata::HumanBinaryData;
use marshal_derive::{Deserialize, DeserializeUpdate, Serialize, SerializeStream, SerializeUpdate};
use marshal_object::derive_variant;
use marshal_serde::WithSerde;
use marshal_update::{hash_map::UpdateHashMap, prim::Prim};
use parking_lot::Mutex;
use uuid::Uuid;
use webauthn_rs::{prelude::Passkey, Webauthn, WebauthnBuilder};

use octant_cookies::{CookieData};
use octant_database::{
    table::{BoxTable, Table},
};
use octant_error::{octant_error, OctantResult};
use octant_server::{
    session::{Session, UrlPrefix},
};


mod into_auth;
mod into_octant;
pub mod login;
pub mod register;
pub mod style;

struct UserId(u64);

// #[derive(Default)]
// struct AccountState {
//     verified_user: Option<UserId>,
// }
//
// #[derive(Default)]
// struct AccountSession {
//     state: AtomicRefCell<AccountState>,
// }
//
// impl SessionData for AccountSession {}
pub static SESSION_COOKIE: &'static str = "__Secure-octant_session";

#[derive(Debug)]
pub struct VerifiedLogin {
    pub email: String,
}

pub struct SessionTable {
    sessions: Mutex<HashMap<Uuid, Arc<VerifiedLogin>>>,
}

#[derive(Serialize, Deserialize, SerializeStream, SerializeUpdate, DeserializeUpdate)]
pub struct Account {
    pub email: Prim<String>,
    pub name: Prim<String>,
    pub passkeys: UpdateHashMap<HumanBinaryData, Prim<WithSerde<Passkey>>>,
}

impl SessionTable {
    pub fn new() -> Arc<Self> {
        Arc::new(SessionTable {
            sessions: Mutex::new(HashMap::new()),
        })
    }
    pub fn get(&self, session: &Session) -> Option<Arc<VerifiedLogin>> {
        let cookie_value = session.data::<CookieData>().get(SESSION_COOKIE)?;
        Some(
            self.sessions
                .lock()
                .get(&Uuid::try_parse(&*cookie_value).ok()?)?
                .clone(),
        )
    }
}

impl Account {
    pub fn new(email: String, name: String) -> Self {
        Account {
            email: Prim::new(email),
            name: Prim::new(name),
            passkeys: UpdateHashMap::new(),
        }
    }
    pub fn add_passkey(&mut self, passkey: Passkey) {
        self.passkeys.insert(
            passkey.cred_id().clone(),
            Prim::new(WithSerde::new(passkey)),
        );
    }
}

#[derive(Default, Serialize, Deserialize, SerializeStream, SerializeUpdate, DeserializeUpdate)]
pub struct AccountTable {
    pub users: UpdateHashMap<String, Account>,
}

derive_variant!(BoxTable, AccountTable);
impl Table for AccountTable {}

fn build_webauthn(session: &Rc<Session>) -> OctantResult<Webauthn> {
    let url = session.try_data::<UrlPrefix>()?.url();
    let host = url.host();
    let rp_id = host
        .ok_or_else(|| octant_error!("host not included in URL"))?
        .to_string();
    let rp_origin = url.join("/")?;
    let webauthn = WebauthnBuilder::new(&rp_id, &rp_origin)?
        .set_user_presence_only_passkeys(true)
        .build()?;
    Ok(webauthn)
}

// pub struct AccountModule {
//     db: ArcDatabase,
//     cookies: Arc<CookieRouter>,
//     sessions: Arc<SessionTable>,
// }
//
// impl AccountModule {
//     pub async fn new(
//         db: ArcDatabase,
//         cookies: Arc<CookieRouter>,
//         sessions: Arc<SessionTable>,
//     ) -> Self {
//         db.write().await.table::<AccountTable>();
//         AccountModule {
//             db,
//             cookies,
//             sessions,
//         }
//     }
//     pub fn register(&self, server: &mut OctantServer) {
//         // todo!();
//         // server.add_handler(LoginHandler {
//         //     db: self.db.clone(),
//         //     cookies: self.cookies.clone(),
//         //     sessions: self.sessions.clone(),
//         // })
//     }
// }
