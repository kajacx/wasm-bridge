use js_sys::Function;

use crate::{direct_bytes::ModuleMemory, DropHandles};

#[derive(Debug, Clone)]
pub struct Func {
    pub(crate) function: Function,
    pub(crate) post_return: Option<Function>,
    pub(crate) memory: ModuleMemory,
    _drop_handles: DropHandles,
}

impl Func {
    pub(crate) fn new(
        function: Function,
        post_return: Option<Function>,
        memory: ModuleMemory,
        drop_handles: DropHandles,
    ) -> Self {
        Self {
            function,
            post_return,
            memory,
            _drop_handles: drop_handles,
        }
    }
}
