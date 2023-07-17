use js_sys::{Function, WebAssembly};

use crate::*;

use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct TypedFunc<'a, Params, Results> {
    _phantom: PhantomData<fn(params: Params) -> Results>,
    instance: &'a WebAssembly::Instance,
    function: Function,
}

impl<'a, Params: ToJsValue, Results: FromJsValue> TypedFunc<'a, Params, Results> {
    pub(crate) fn new(instance: &'a WebAssembly::Instance, function: Function) -> Self {
        Self {
            _phantom: PhantomData,
            instance,
            function,
        }
    }

    pub fn call(&self, _store: impl AsContextMut, params: Params) -> Result<Results, Error> {
        let args = params.to_function_args();
        let result = self.function.apply(self.instance.as_ref(), &args);
        Results::from_fn_result(&result)
    }
}
