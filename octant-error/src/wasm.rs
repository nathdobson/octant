use std::fmt::{Debug, Display, Formatter};

use sendable::SendOption;
use wasm_bindgen::JsValue;
use web_sys::console;

use crate::OctantError;

#[derive(Debug)]
struct WasmError {
    value: SendOption<JsValue>,
}

impl Display for WasmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl std::error::Error for WasmError {}

impl WasmError {
    pub fn new(x: JsValue) -> Self {
        WasmError {
            value: SendOption::new(Some(x)),
        }
    }
    pub fn as_ref(&self) -> Option<&JsValue> {
        self.value.as_ref()
    }
}

pub fn log_error(x: &OctantError) {
    log::error!("{}", x);
    if let Some(wasm) = x.0.downcast_ref::<WasmError>() {
        if let Some(js) = wasm.as_ref() {
            console::error_1(js);
        } else {
            log::error!("Error object could not be logged because it crossed a thread boundary.");
        }
    }
}

// not sure why SendOption doesn't do this.
unsafe impl Sync for WasmError {}

// impl From<WasmError> for OctantError {
//     fn from(value: WasmError) -> Self {
//         Self::from(OctantError::from(value))
//     }
// }

impl From<JsValue> for OctantError {
    fn from(value: JsValue) -> Self {
        OctantError(anyhow::Error::new(WasmError::new(value)))
    }
}
