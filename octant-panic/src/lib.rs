#![feature(panic_update_hook)]

use std::{
    cell::Cell,
    panic::{catch_unwind, PanicInfo, UnwindSafe, update_hook},
    sync::Once,
};

use octant_error::{octant_error, OctantError, OctantResult};

thread_local! {
    static LAST_ERROR:Cell<Option<OctantError>> = Cell::new(None);
}
pub fn catch_error<T>(f: impl UnwindSafe + FnOnce() -> T) -> OctantResult<T> {
    catch_unwind(f).map_err(|e| {
        if let Some(error) = LAST_ERROR.take() {
            return error;
        }
        let e = match e.downcast::<String>() {
            Ok(e) => return OctantError::msg(*e),
            Err(e) => e,
        };
        let _ = match e.downcast::<&str>() {
            Ok(e) => return OctantError::msg(*e),
            Err(e) => e,
        };
        octant_error!("unknown error")
    })
}

static REGISTER: Once = Once::new();

pub fn register_handler() {
    REGISTER.call_once(|| update_hook(Box::new(panic_handler)))
}

pub fn panic_handler(
    prev: &(dyn Fn(&PanicInfo<'_>) + Send + Sync + 'static),
    info: &PanicInfo<'_>,
) {
    prev(info);
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
    let error = OctantError::msg(message);
    LAST_ERROR.set(Some(error));
}
