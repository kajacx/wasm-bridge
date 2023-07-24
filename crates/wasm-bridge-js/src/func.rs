use anyhow::bail;
use js_sys::{Function, WebAssembly};

use crate::{helpers::map_js_error, AsContextMut, Error, FromJsValue, ToJsValue, Val};

pub struct Func {
    instance: WebAssembly::Instance,
    function: Function,
}

impl Func {
    pub fn new(instance: WebAssembly::Instance, function: Function) -> Self {
        Func { instance, function }
    }

    pub fn call(
        &self,
        _: impl AsContextMut,
        params: &[Val],
        results: &mut [Val],
    ) -> Result<(), Error> {
        let params = params
            .iter()
            .map(|val| val.to_js_value())
            .collect::<js_sys::Array>();
        let result = self
            .function
            .apply(&self.function, &params)
            .map_err(map_js_error("Exported function threw an exception"))?;
        let data = if result.is_array() {
            Vec::<Val>::from_js_value(&result)?
        } else if result.is_undefined() || result.is_null() {
            vec![]
        } else {
            vec![Val::from_js_value(&result)?]
        };
        if data.len() != results.len() {
            bail!(
                "Exported function {} should have {} arguments, but it has {} instead.",
                self.function.name().as_string().unwrap_or_default(),
                results.len(),
                data.len()
            );
        }
        for (target, source) in results.iter_mut().zip(data.into_iter()) {
            *target = source;
        }
        Ok(())
    }
}
