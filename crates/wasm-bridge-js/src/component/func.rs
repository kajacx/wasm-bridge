use js_sys::Function;

use crate::{AsContextMut, FuncId};

#[derive(Debug, Clone, Copy)]
pub struct Func {
    func_id: FuncId,
}
// TODO: remove func from store on drop

impl Func {
    pub(crate) fn new(func_id: FuncId) -> Self {
        Self { func_id }
    }
}
