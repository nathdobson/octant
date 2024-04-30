#![feature(trait_upcasting)]
#![allow(dead_code)]

use atomic_refcell::AtomicRefCell;

use octant_server::session::SessionData;

mod into_auth;
mod into_octant;
pub mod register;

struct UserId(u64);

#[derive(Default)]
struct LoginState {
    verified_user: Option<UserId>,
}

#[derive(Default)]
struct LoginSession {
    state: AtomicRefCell<LoginState>,
}

impl SessionData for LoginSession {}
