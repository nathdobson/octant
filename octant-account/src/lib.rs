#![feature(trait_upcasting)]
#![allow(dead_code)]
#![feature(arbitrary_self_types)]
#![allow(unused_variables)]

use anyhow::anyhow;
use atomic_refcell::AtomicRefCell;
use url::Url;
use webauthn_rs::{Webauthn, WebauthnBuilder};

use octant_database::value::{dict::Dict, prim::Prim};
use octant_server::session::SessionData;

mod into_auth;
mod into_octant;
pub mod login;
pub mod register;

struct UserId(u64);

#[derive(Default)]
struct AccountState {
    verified_user: Option<UserId>,
}

#[derive(Default)]
struct AccountSession {
    state: AtomicRefCell<AccountState>,
}

impl SessionData for AccountSession {}

octant_database::database_struct! {
    pub struct Account{
        email: Prim<String>,
        name: Prim<String>,
    }
}

octant_database::database_struct! {
    #[derive(Default)]
    pub struct AccountDatabase{
        users: Dict<String, Account>,
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
