use js_sys::{Array, Function, WebAssembly};
use wasm_bindgen::JsValue;

use crate::*;

use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct TypedFunc<'a, Params, Results> {
    _phantom: PhantomData<fn(params: Params) -> Results>,
    instance: &'a WebAssembly::Instance,
    function: Function,
}

impl<'a, Params: Into<JsValue>, Results: FromJsValue> TypedFunc<'a, Params, Results> {
    pub(crate) fn new(instance: &'a WebAssembly::Instance, function: Function) -> Self {
        Self {
            _phantom: PhantomData,
            instance,
            function,
        }
    }

    pub fn call(&self, _store: &Store<()>, params: Params) -> Result<Results, Error> {
        let as_js_value = params.into();
        let args = Array::of1(&as_js_value);
        let result = self.function.apply(self.instance.as_ref(), &args)?;
        Results::from_js_value(&result)
    }
}
