use std::sync::Arc;

use octant_gui_core::global::GlobalMethod;
use octant_gui_core::HandleId;

use crate::{peer, window};

pub fn invoke_with(method: &GlobalMethod, handle: HandleId) -> Option<Arc<dyn peer::Trait>> {
    match method {
        GlobalMethod::Window => Some(Arc::new(window::Value::new(handle, window().unwrap()))),
    }
}
