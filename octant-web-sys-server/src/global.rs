use crate::window::{ArcWindow, Window, WindowTag, WindowValue};
use catalog::register;
#[cfg(side = "server")]
use octant_gui::{
    runtime::{HasTypedHandle, Runtime},
    ServerContext, UpMessageHandler, UP_MESSAGE_HANDLER_REGISTRY,
};
#[cfg(side = "client")]
use octant_gui_client::{ClientContext, DownMessageHandler, DOWN_MESSAGE_HANDLER_REGISTRY};
use octant_gui_core::{
    define_sys_rpc, DownMessage, NewDownMessage, NewUpMessage, TypedHandle, UpMessage,
    UpMessageList,
};
use octant_serde::define_serde_impl;
use safe_once::sync::OnceLock;
use serde::{Deserialize, Serialize};
use std::{marker::PhantomData, sync::Arc};
use std::hint::must_use;
use wasm_error::WasmError;
use octant_gui_core::FromHandle;

#[cfg(side = "server")]
pub struct Global {
    runtime: Arc<Runtime>,
    window: OnceLock<ArcWindow>,
}

#[cfg(side = "server")]
impl Global {
    pub fn runtime(&self) -> &Arc<Runtime> {
        &self.runtime
    }
    pub fn new(runtime: Arc<Runtime>) -> Arc<Self> {
        Arc::new(Global {
            runtime,
            window: OnceLock::new(),
        })
    }
}

#[cfg(side = "server")]
impl Global {
    pub fn window(&self) -> &ArcWindow {
        self.window.get_or_init(|| window(&self.runtime))
    }
}

define_sys_rpc! {
    fn window(_ctx) -> Window {
        Ok(web_sys::window().unwrap())
    }
}
