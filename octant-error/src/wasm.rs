use std::{
    convert::TryFrom,
    fmt::{Debug, Display, Formatter},
};

use crate::OctantError;
use sendable::SendOption;
use wasm_bindgen::JsValue;
use web_sys::console;

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

impl TryFrom<anyhow::Error> for WasmError {
    type Error = anyhow::Error;

    fn try_from(x: anyhow::Error) -> Result<WasmError, anyhow::Error> {
        x.downcast::<WasmError>()
    }
}

pub fn log_error(x: &anyhow::Error) {
    log::error!("{}", x);
    if let Some(wasm) = x.downcast_ref::<WasmError>() {
        if let Some(js) = wasm.as_ref() {
            console::error_1(js);
        } else {
            log::error!("Error object could not be logged because it crossed a thread boundary.");
        }
    }
}

// not sure why SendOption doesn't do this.
unsafe impl Sync for WasmError {}

impl From<WasmError> for OctantError {
    fn from(value: WasmError) -> Self {
        Self::from(anyhow::Error::from(value))
    }
}

impl From<JsValue> for OctantError {
    fn from(value: JsValue) -> Self {
        OctantError(anyhow::Error::new(WasmError::new(value)))
    }
}
