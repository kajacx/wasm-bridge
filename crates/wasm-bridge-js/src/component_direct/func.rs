use js_sys::Function;

use crate::direct_bytes::ModuleMemory;

#[derive(Debug, Clone)]
pub struct Func {
    pub(crate) function: Function,
    pub(crate) post_return: Option<Function>,
    pub(crate) memory: ModuleMemory,
}

impl Func {
    pub(crate) fn new(
        function: Function,
        post_return: Option<Function>,
        memory: ModuleMemory,
    ) -> Self {
        Self {
            function,
            post_return,
            memory,
        }
    }
}
