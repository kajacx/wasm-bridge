use std::{cell::Cell, marker::PhantomData};

use anyhow::Context;
use wasm_bindgen::JsValue;

use crate::{
    direct_bytes::{Lift, LowerArgs, ModuleWriteableMemory},
    helpers::map_js_error,
    AsContextMut, FromJsValue, Memory, Result, ToJsValue,
};

use super::Func;

pub struct TypedFunc<Params, Return> {
    func: Func,
    post_return_arg: Cell<JsValue>,
    _phantom: PhantomData<dyn Fn(Params) -> Return>,
}

impl<Params, Return> TypedFunc<Params, Return> {
    pub fn new(func: Func) -> Self {
        Self {
            func,
            post_return_arg: Cell::new(JsValue::UNDEFINED),
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
        Return: Lift,
        Return::Abi: FromJsValue, // TODO: quick ugly hack
    {
        let arguments = params.to_fn_args(&self.func.memory);
        let result = self
            .func
            .function
            .apply(&JsValue::UNDEFINED, &arguments)
            .map_err(map_js_error("Error inside exported function"))?;
        let result_abi = Return::Abi::from_js_value(&result)
            .context("Cannot cast return type to correct ABI type")?;
        Lift::from_abi(result_abi, &self.func.memory)
    }

    pub fn call_async(&self, store: impl AsContextMut, params: Params) -> Result<Return>
    where
        Params: LowerArgs,
        Return: Lift,
        Return::Abi: FromJsValue, // TODO: quick ugly hack
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
