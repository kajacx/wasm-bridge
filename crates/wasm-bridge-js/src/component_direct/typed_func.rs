use std::{cell::Cell, marker::PhantomData};

use anyhow::Context;
use js_sys::Array;
use wasm_bindgen::JsValue;

use crate::{
    direct_bytes::{Lift, Lower, WriteableMemory},
    helpers::map_js_error,
    AsContextMut, Result,
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
        Params: Lower,
        Return: Lift,
    {
        // TODO: re-use same vec and JS array?
        // Local static variable should be different for each monomorphization?
        let memory = &self.func.memory;
        let num_args = Params::num_args();

        let arguments = if num_args <= 16 {
            let mut args = Vec::<JsValue>::new();
            params.to_abi(&mut args, &memory);
            args.into_iter().collect()
        } else {
            let mut buffer = memory.allocate(Params::alignment(), Params::flat_byte_size())?;
            params.write_to(&mut buffer, memory)?;
            let addr = memory.flush(buffer) as u32;
            Array::of1(&addr.into())
        };

        let result_js = self
            .func
            .function
            .apply(&JsValue::UNDEFINED, &arguments)
            .map_err(map_js_error("Error inside exported function"))?;

        let result = Return::from_js_return(&result_js, &self.func.memory)
            .context("Cannot cast return type to correct ABI type")?;

        self.post_return_arg.set(result_js);

        Ok(result)
    }

    pub fn call_async(&self, store: impl AsContextMut, params: Params) -> Result<Return>
    where
        Params: Lower,
        Return: Lift,
    {
        self.call(store, params)
    }

    pub fn post_return(&self, _store: impl AsContextMut) -> Result<()> {
        if let Some(func) = &self.func.post_return {
            func.call1(
                &JsValue::UNDEFINED,
                &self.post_return_arg.replace(JsValue::UNDEFINED),
            )
            .map_err(map_js_error("Call post_return"))?;
        }
        Ok(())
    }

    pub fn post_return_async(&self, store: impl AsContextMut) -> Result<()> {
        self.post_return(store)
    }
}
