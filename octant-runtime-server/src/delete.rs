use std::rc::Rc;
use octant_runtime_derive::rpc;
use crate::{ handle::RawHandle};
use crate::runtime::Runtime;

#[rpc]
pub fn delete_rpc(runtime: &Rc<Runtime>, handle:RawHandle){
    runtime.delete(handle);
    Ok(())
}