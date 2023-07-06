use js_sys::Function;

use crate::{AsContext, AsContextMut, FuncId};

#[derive(Debug, Clone, Copy)]
pub struct Func {
    func_id: FuncId,
}
// TODO: remove func from store on drop

impl Func {
    pub(crate) fn new(func_id: FuncId) -> Self {
        Self { func_id }
    }

    pub(crate) fn function<'a>(&'a self, store: &'a impl AsContext) -> &'a Function {
        store.as_context().get_function(self.func_id).unwrap()
    }
}
