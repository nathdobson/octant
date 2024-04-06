use std::{
    cell::Cell,
    panic::{catch_unwind, set_hook, PanicInfo, UnwindSafe},
    sync::Once,
};

use anyhow::anyhow;

thread_local! {
    static LAST_ERROR:Cell<Option<anyhow::Error>> = Cell::new(None);
}
pub fn catch_error<T>(f: impl UnwindSafe + FnOnce() -> T) -> anyhow::Result<T> {
    catch_unwind(f).map_err(|e| {
        if let Some(error) = LAST_ERROR.take() {
            return error;
        }
        let e = match e.downcast::<String>() {
            Ok(e) => return anyhow::Error::msg(*e),
            Err(e) => e,
        };
        let _ = match e.downcast::<&str>() {
            Ok(e) => return anyhow::Error::msg(*e),
            Err(e) => e,
        };
        anyhow!("unknown error")
    })
}

static REGISTER: Once = Once::new();

pub fn register_handler() {
    REGISTER.call_once(|| set_hook(Box::new(panic_handler)))
}

pub fn panic_handler(info: &PanicInfo<'_>) {
    let payload = if let Some(payload) = info.payload().downcast_ref::<String>() {
        payload
    } else if let Some(payload) = info.payload().downcast_ref::<&str>() {
        payload
    } else {
        "Unknown payload type"
    };
    let message = if let Some(location) = info.location() {
        format!("{}\nat {}", payload, location)
    } else {
        format!("{}", payload)
    };
    let error = anyhow::Error::msg(message);
    LAST_ERROR.set(Some(error));
}
