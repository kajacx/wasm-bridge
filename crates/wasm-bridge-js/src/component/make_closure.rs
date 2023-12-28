// use std::future::Future;
use std::rc::Rc;

use wasm_bindgen::{prelude::*, JsValue};

use crate::conversions::FromJsValue;
use crate::direct::*;
use crate::{DataHandle, DropHandle, Result, StoreContextMut};
use js_sys::{Array, Function};

pub(crate) type MakeClosure<T> = Box<dyn Fn(DataHandle<T>, ModuleMemory) -> (JsValue, DropHandle)>;

pub trait IntoMakeClosure<T, Params, Results> {
    fn into_make_closure(self) -> MakeClosure<T>;
}

impl<T, P, R, F> IntoMakeClosure<T, P, R> for F
where
    T: 'static,
    P: Lift + 'static,
    R: Lower + 'static,
    F: Fn(StoreContextMut<T>, P) -> Result<R> + 'static,
{
    fn into_make_closure(self) -> MakeClosure<T> {
        let self_rc = Rc::new(self);

        let make_closure = move |handle: DataHandle<T>, memory: ModuleMemory| {
            let self_clone = self_rc.clone();

            let closure =
                Closure::<dyn Fn(Array) -> Result<JsValue, JsValue>>::new(move |args: Array| {
                    let mut args_iter = JsArgsReader::new(args);
                    let args = if P::NUM_ARGS <= 16 {
                        P::from_js_args(&mut args_iter, &memory).map_err(|err| {
                            format!("conversion of imported fn arguments: {err:?}")
                        })?
                    } else {
                        let addr = args_iter
                            .next()
                            .ok_or("getting pointer to imported fn args")?;
                        P::from_js_ptr_return(&addr, &memory)
                            .map_err(|err| format!("from js ptr return: {err:?}"))?
                    };

                    let result = self_clone(&mut handle.borrow_mut(), args)
                        .map_err(|err| format!("host imported fn returned error: {err:?}"))?;

                    if R::NUM_ARGS <= 1 {
                        let result = result
                            .to_js_return(&memory)
                            .map_err(|err| format!("conversion of imported fn result: {err:?}"))?;
                        Ok(result)
                    } else {
                        let addr = args_iter
                            .next()
                            .ok_or("missing last mem address argument")?;
                        let addr = u32::from_js_value(&addr)
                            .map_err(|err| format!("return address is not a number: {err:?}"))?
                            as usize;

                        // Buffer is already allocated, we just write there
                        let mut buffer = ByteBuffer::new(addr, R::BYTE_SIZE);
                        result.write_to(&mut buffer, &memory).map_err(|err| {
                            format!("failed to write result of an imported function: {err:?}")
                        })?;
                        memory.flush(buffer);

                        Ok(JsValue::UNDEFINED)
                    }
                });

            let (function, drop_handle) = DropHandle::from_closure(closure);
            (inflate_js_fn_args(&function), drop_handle)
        };

        Box::new(make_closure)
    }
}

/**
 * Takes a JS function that takes one Array argument
 * and returns a JS function that takes many arguments,
 * but calls the original function with those arguments.
 */
fn inflate_js_fn_args(function: &JsValue) -> JsValue {
    let converter: Function = js_sys::eval("(inner_fn) => (...outer_args) => inner_fn(outer_args)")
        .expect("eval converter")
        .into();

    converter
        .call1(&JsValue::UNDEFINED, function)
        .expect("call converter")
}
