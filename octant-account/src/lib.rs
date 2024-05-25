#![feature(trait_upcasting)]
#![allow(dead_code)]
#![feature(arbitrary_self_types)]
#![allow(unused_variables)]
#![deny(unused_must_use)]

use std::{collections::HashMap};
use std::sync::Arc;

use anyhow::anyhow;
use base64urlsafedata::HumanBinaryData;
use parking_lot::Mutex;
use url::Url;
use uuid::Uuid;
use webauthn_rs::{prelude::Passkey, Webauthn, WebauthnBuilder};

use octant_database::{
    tack::Tack,
    value::{dict::Dict, prim::Prim},
};
use octant_server::{cookies::CookieData, session::Session};

mod into_auth;
mod into_octant;
pub mod login;
pub mod register;

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

octant_database::database_struct! {
    pub struct Account {
        pub email: Prim<String>,
        pub name: Prim<String>,
        pub passkeys: Dict<HumanBinaryData,Prim<Passkey>>,
    }
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
        Account::new_raw(Prim::new(email), Prim::new(name), Dict::new())
    }
    pub fn add_passkey(self: Tack<Self>, passkey: Passkey) {
        self.passkeys()
            .insert(passkey.cred_id().clone(), Prim::new(passkey));
    }
}

octant_database::database_struct! {
    #[derive(Default)]
    pub struct AccountDatabase{
        pub users: Dict<String, Account>,
    }
}

fn build_webauthn(url: &Url) -> anyhow::Result<Webauthn> {
    let rp_id = url
        .host()
        .ok_or_else(|| anyhow!("host not included in URL"))?
        .to_string();
    let rp_origin = url.join("/")?;
    let webauthn = WebauthnBuilder::new(&rp_id, &rp_origin)?
        .set_user_presence_only_passkeys(true)
        .build()?;
    Ok(webauthn)
}
