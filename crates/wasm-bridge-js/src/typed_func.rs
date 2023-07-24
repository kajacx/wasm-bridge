use js_sys::{Function, WebAssembly};

use crate::*;

use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct TypedFunc<Params, Results> {
    _phantom: PhantomData<fn(params: Params) -> Results>,
    instance: WebAssembly::Instance,
    function: Function,
}

impl<Params: ToJsValue, Results: FromJsValue> TypedFunc<Params, Results> {
    pub(crate) fn new(instance: WebAssembly::Instance, function: Function) -> Self {
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
