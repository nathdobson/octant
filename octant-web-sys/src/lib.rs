#![deny(unused_must_use)]
#![allow(unused_imports)]

use catalog::register;
#[cfg(feature = "server")]
use octant_gui::{
    runtime::{HasTypedHandle, Runtime},
    window::ArcWindow,
    ServerContext, UpMessageHandler, UP_MESSAGE_HANDLER_REGISTRY,
};
#[cfg(feature = "client")]
use octant_gui_client::{ClientContext, DownMessageHandler, DOWN_MESSAGE_HANDLER_REGISTRY};
use octant_gui_core::{
    DownMessage, NewDownMessage, NewUpMessage, TypedHandle, UpMessage, UpMessageList, WindowTag,
};
use octant_serde::define_serde_impl;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use wasm_error::WasmError;

#[derive(Serialize, Deserialize, Debug)]
pub struct PromptRequest {
    window: TypedHandle<WindowTag>,
    request: String,
}

define_serde_impl!(PromptRequest: NewDownMessage);
impl NewDownMessage for PromptRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct PromptResponse {
    response: Option<String>,
}

define_serde_impl!(PromptResponse: NewUpMessage);
impl NewUpMessage for PromptResponse {}

#[cfg(feature = "server")]
pub fn prompt(runtime: &Arc<Runtime>, window: &ArcWindow, request: String) {
    runtime.send(DownMessage::NewDownMessage(Box::new(PromptRequest {
        window: window.typed_handle(),
        request,
    })));
}

#[cfg(feature = "client")]
#[register(DOWN_MESSAGE_HANDLER_REGISTRY)]
fn handle_prompt() -> DownMessageHandler<PromptRequest> {
    |ctx: ClientContext, req: PromptRequest| {
        let window = ctx.runtime.handle(req.window);
        let response = window
            .native()
            .prompt_with_message(&req.request)
            .map_err(WasmError::new)?;
        ctx.runtime.send(UpMessageList {
            commands: vec![UpMessage::NewUpMessage(Box::new(PromptResponse {
                response,
            }))],
        })?;
        Ok(())
    }
}

#[cfg(feature = "server")]
#[register(UP_MESSAGE_HANDLER_REGISTRY)]
fn handle_prompt_result() -> UpMessageHandler<PromptResponse> {
    |_: ServerContext, resp: PromptResponse| {
        log::info!("Response is {:?}", resp);
        Ok(())
    }
}
