use std::marker::PhantomData;

use wasm_bindgen::JsValue;

use crate::{AsContextMut, FromJsResults, FromJsValue, Result, ToJsParams};

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

    pub fn call(&self, store: impl AsContextMut, params: Params) -> Result<Return>
    where
        Params: ToJsParams,
        Return: FromJsResults,
    {
        let argument = params.to_js_params();
        let results = self
            .func
            .function(&store)
            .apply(&JsValue::UNDEFINED, &argument)?;
        Return::from_js_results(&results)
    }

    pub fn post_return(&self, _store: impl AsContextMut) -> Result<()> {
        Ok(())
    }
}

// pub trait FromJsAbi: Sized {
//     fn from_js_abi(value: JsValue) -> Result<Self>;
// }

// pub trait IntoJsAbi {
//     fn into_js_abi(self) -> JsValue;
// }

// impl FromJsAbi for (String,) {
//     fn from_js_abi(value: JsValue) -> Result<Self> {
//         Ok((value.as_string().unwrap(),)) // TODO: add user error
//     }
// }

// impl IntoJsAbi for (&str,) {
//     fn into_js_abi(self) -> JsValue {
//         self.0.into()
//     }
// }
