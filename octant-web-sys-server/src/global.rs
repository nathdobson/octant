use crate::window::{ArcWindow, WindowTag, WindowValue};
use catalog::register;
#[cfg(side = "server")]
use octant_gui::{
    runtime::{HasTypedHandle, Runtime},
    ServerContext, UpMessageHandler, UP_MESSAGE_HANDLER_REGISTRY,
};
#[cfg(side = "client")]
use octant_gui_client::{ClientContext, DownMessageHandler, DOWN_MESSAGE_HANDLER_REGISTRY};
use octant_gui_core::{
    DownMessage, NewDownMessage, NewUpMessage, TypedHandle, UpMessage, UpMessageList,
};
use octant_serde::define_serde_impl;
use safe_once::sync::OnceLock;
use serde::{Deserialize, Serialize};
use std::{marker::PhantomData, sync::Arc};
use wasm_error::WasmError;

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
        self.window.get_or_init(|| {
            let window = Arc::new(WindowValue::new(self.runtime.add_uninit()));
            self.runtime
                .send(DownMessage::NewDownMessage(Box::new(WindowRequest {
                    window: window.typed_handle(),
                })));
            window
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WindowRequest {
    window: TypedHandle<WindowTag>,
}

define_serde_impl!(WindowRequest: NewDownMessage);
impl NewDownMessage for WindowRequest {}

#[cfg(side = "client")]
#[register(DOWN_MESSAGE_HANDLER_REGISTRY)]
fn handle_prompt() -> DownMessageHandler<WindowRequest> {
    |ctx: ClientContext, req: WindowRequest| {
        ctx.runtime.add(
            req.window,
            Arc::new(WindowValue::new(req.window.0, web_sys::window().unwrap())),
        );
        Ok(())
    }
}
