use std::sync::Arc;

use octant_gui_core::global::GlobalMethod;
use octant_gui_core::Method;

use crate::{window, Window};
use crate::runtime::Runtime;

pub struct Global {
    root: Arc<Runtime>,
    window: Window,
}

impl Global {
    pub fn new(root: Arc<Runtime>) -> Arc<Self> {
        Arc::new(Global {
            root: root.clone(),
            window: root.add(window::Value::new(root.invoke(Method::Global(GlobalMethod::Window)))),
        })
    }
    pub fn root(&self) -> &Arc<Runtime> {
        &self.root
    }
    pub fn window(&self) -> &Window {
        &self.window
    }
}
