use js_sys::Function;
use wasm_bindgen::JsValue;

use crate::*;

use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct TypedFunc<Params, Results> {
    _phantom: PhantomData<fn(params: Params) -> Results>,
    function: Function,
}

impl<Params: ToJsValue, Results: FromJsValue> TypedFunc<Params, Results> {
    pub(crate) fn new(function: Function) -> Self {
        Self {
            _phantom: PhantomData,
            function,
        }
    }

    pub fn call(&self, _store: impl AsContextMut, params: Params) -> Result<Results, Error> {
        let args = params.to_function_args();
        let result = self.function.apply(&JsValue::UNDEFINED, &args);
        Results::from_fn_result(&result)
    }
}
