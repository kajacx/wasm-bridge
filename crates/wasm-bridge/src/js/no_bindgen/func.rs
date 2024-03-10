use std::rc::Rc;

use anyhow::bail;
use js_sys::{Array, Function, Reflect};
use wasm_bindgen::JsValue;

use crate::{helpers::map_js_error, *};

pub struct Func {
    function: Function,
    _closures: Rc<Vec<DropHandle>>,
}

impl Func {
    pub(crate) fn new(function: Function, closures: Rc<Vec<DropHandle>>) -> Self {
        Self {
            function,
            _closures: closures,
        }
    }

    pub fn call(&self, _store: impl AsContextMut, args: &[Val], rets: &mut [Val]) -> Result<()> {
        if self.function.length() != args.len() as u32 {
            bail!(
                "Exported function takes {} arguments, but {} arguments were provided instead",
                self.function.length(),
                args.len()
            );
        }

        let js_args: Array = args.iter().map(Val::to_js_value).collect();

        let js_rets = self
            .function
            .apply(&JsValue::UNDEFINED, &js_args)
            .map_err(map_js_error("call untyped exported function"))?;

        match rets.len() {
            0 => {
                <()>::from_js_value(&js_rets)?;
            }
            1 => {
                rets[0] = Val::from_js_value(&js_rets)?;
            }
            n => {
                if !js_rets.is_array() {
                    return Err(map_js_error("Exported function did not return an array")(
                        js_rets,
                    ));
                }
                let js_array: Array = js_rets.into();
                if js_array.length() != n as u32 {
                    bail!(
                        "Exported function returned {} values, but {} result slots were provided",
                        js_array.length(),
                        n
                    );
                }

                for (index, ret) in rets.iter_mut().enumerate() {
                    let js_val = Reflect::get_u32(&js_array, index as _)
                        .map_err(map_js_error("set rets at index"))?;

                    *ret = Val::from_js_value(&js_val)?;
                }
            }
        }

        Ok(())
    }
}
