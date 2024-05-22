use crate::{define_sys_rpc, handle::RawHandle};
define_sys_rpc! {
    pub fn delete_rpc(runtime, handle: RawHandle) -> () {
        runtime.delete(handle);
        Ok(())
    }
}
