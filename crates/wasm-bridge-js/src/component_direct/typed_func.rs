use std::marker::PhantomData;

use wasm_bindgen::JsValue;

use crate::{
    direct_bytes::{LowerArgs, ModuleWriteableMemory},
    AsContextMut, FromJsValue, Memory, Result, ToJsValue,
};

use super::Func;

pub struct TypedFunc<Params, Return> {
    func: Func,

    _phantom: PhantomData<dyn Fn(Params) -> Return>,
}

impl<Params, Return> TypedFunc<Params, Return> {
    pub fn new(func: Func) -> Self {
        Self {
            func,

            _phantom: PhantomData,
        }
    }

    /// # Safety
    /// This function is 100% safe, it just needs to match wasmtime's API
    pub unsafe fn new_unchecked(func: Func) -> Self {
        Self::new(func)
    }

    pub fn func(&self) -> &Func {
        &self.func
    }

    pub fn call(&self, _store: impl AsContextMut, params: Params) -> Result<Return>
    where
        Params: LowerArgs,
        Return: FromJsValue,
    {
        let arguments = params.to_fn_args(&self.func.memory);
        let result = self.func.function.apply(&JsValue::UNDEFINED, &arguments);
        Return::from_fn_result(&result)
    }

    pub fn call_async(&self, store: impl AsContextMut, params: Params) -> Result<Return>
    where
        Params: LowerArgs,
        Return: FromJsValue,
    {
        self.call(store, params)
    }

    pub fn post_return(&self, _store: impl AsContextMut) -> Result<()> {
        Ok(())
    }

    pub fn post_return_async(&self, store: impl AsContextMut) -> Result<()> {
        self.post_return(store)
    }
}
