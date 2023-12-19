use std::rc::Rc;

use js_sys::Function;

use crate::{direct_bytes::ModuleWriteableMemory, DropHandle, Memory};

#[derive(Debug, Clone)]
pub struct Func {
    pub(crate) function: Function,
    pub(crate) memory: ModuleWriteableMemory,
}

impl Func {
    pub(crate) fn new(function: Function, memory: ModuleWriteableMemory) -> Self {
        Self { function, memory }
    }
}
