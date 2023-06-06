use js_sys::{Array, Function, WebAssembly};

use crate::*;

use std::marker::PhantomData;

pub struct TypedFunc<'a, Params, Results> {
    _phantom: PhantomData<fn(params: Params) -> Results>,
    instance: &'a WebAssembly::Instance,
    function: Function,
}

impl<'a> TypedFunc<'a, i32, i32> {
    pub(crate) fn new(instance: &'a WebAssembly::Instance, function: Function) -> Self {
        Self {
            _phantom: PhantomData,
            instance,
            function,
        }
    }

    pub fn call(&self, _store: &Store<()>, params: i32) -> Result<i32, Error> {
        let as_js_value = wasm_bindgen::JsValue::from(params);
        let args = Array::of1(&as_js_value);
        let result = self
            .function
            .apply(self.instance.as_ref(), &args)
            .expect("TODO: call function");
        Ok(result.as_f64().unwrap() as _)
    }
}
