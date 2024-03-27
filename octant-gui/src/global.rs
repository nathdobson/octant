use std::sync::Arc;

use octant_gui_core::global::GlobalMethod;
use octant_gui_core::Method;

use crate::{window, Window};
use crate::runtime::Runtime;

pub struct Global {
    runtime: Arc<Runtime>,
    window: Window,
}

impl Global {
    pub fn new(root: Arc<Runtime>) -> Arc<Self> {
        Arc::new(Global {
            runtime: root.clone(),
            window: root.add(window::Value::new(
                root.invoke(Method::Global(GlobalMethod::Window)),
            )),
        })
    }
    pub fn runtime(&self) -> &Arc<Runtime> {
        &self.runtime
    }
    pub fn window(&self) -> &Window {
        &self.window
    }
}
