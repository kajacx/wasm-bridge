// use std::future::Future;
use std::rc::Rc;

use wasm_bindgen::{prelude::*, JsValue};

use crate::conversions::FromJsValue;
use crate::direct_bytes::{ByteBuffer, JsArgsReader, Lift, Lower, ModuleMemory, WriteableMemory};
use crate::{DataHandle, DropHandle, Result, StoreContextMut};
use js_sys::{Array, Function};

pub(crate) type MakeClosure<T> = Box<dyn Fn(DataHandle<T>, ModuleMemory) -> (JsValue, DropHandle)>;

pub trait IntoMakeClosure<T, Params, Results> {
    fn into_make_closure(self) -> MakeClosure<T>;
}

impl<T, P0, P1, R, F> IntoMakeClosure<T, (P0, P1), R> for F
where
    T: 'static,
    P0: Lift + 'static,
    P1: Lift + 'static,
    R: Lower + 'static,
    F: Fn(StoreContextMut<T>, (P0, P1)) -> Result<R> + 'static,
{
    fn into_make_closure(self) -> MakeClosure<T> {
        let self_rc = Rc::new(self);

        let make_closure = move |handle: DataHandle<T>, memory: ModuleMemory| {
            let self_clone = self_rc.clone();

            let closure =
                Closure::<dyn Fn(Array) -> Result<JsValue, JsValue>>::new(move |args: Array| {
                    // FIXME: if "flat" argument size is > 16 values, args will contain a pointer to the data instead
                    let mut args_iter = JsArgsReader::new(args);
                    let args = <(P0, P1)>::from_js_args(&mut args_iter, &memory)
                        .map_err(|err| format!("conversion of imported fn arguments: {err:?}"))?;

                    let result = self_clone(&mut handle.borrow_mut(), args)
                        .map_err(|err| format!("host imported fn returned error: {err:?}"))?;

                    if R::num_args() <= 1 {
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
                        let mut buffer = ByteBuffer::new(addr, R::FLAT_BYTE_SIZE);
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

macro_rules! make_closure {
    ($(($param: ident, $name: ident)),*) => {
        impl<T, $($name, )* R, F> IntoMakeClosure<T, ($($name,)*), R> for F
        where
            T: 'static,
            $($name: Lift + 'static,)*
            R: Lower + 'static ,
            F: Fn(StoreContextMut<T>, ($($name, )*)) -> Result<R> + 'static,
        {
            fn into_make_closure(self) -> MakeClosure<T> {
                let self_rc = Rc::new(self);

                let make_closure = move |handle: DataHandle<T>, memory: ModuleMemory| {
                    let self_clone = self_rc.clone();

                    let closure =
                        Closure::<dyn Fn(Array) -> Result<JsValue, JsValue>>::new(move |args: Array| {
                            let mut args_iter = JsArgsReader::new(args);
                            let args = <($($name,)*)>::from_js_args(&mut args_iter, &memory).map_err(|err| format!("conversion of imported fn arguments: {err:?}"))?;

                            let result = self_clone(
                                &mut handle.borrow_mut(),
                                args
                            ).map_err(|err| format!("host imported fn returned error: {err:?}"))?;

                            if R::num_args() <= 1 {
                                let result = result
                                    .to_js_return(&memory)
                                    .map_err(|err| format!("conversion of imported fn result: {err:?}"))?;
                                Ok(result)
                            } else {
                                let addr = args_iter.next().ok_or("missing last mem address argument")?;
                                let addr = u32::from_js_value(&addr).map_err(|err| format!("return address is not a number: {err:?}"))? as usize;

                                // Buffer is already allocated, we just write there
                                let mut buffer = ByteBuffer::new(addr, R::FLAT_BYTE_SIZE);
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
    };
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

#[rustfmt::skip]
make_closure!();
#[rustfmt::skip]
make_closure!((p0, P0));
#[rustfmt::skip]
// make_closure!((p0, P0), (p1, P1));
#[rustfmt::skip]
make_closure!((p0, P0), (p1, P1), (p2, P2));
// #[rustfmt::skip]
// make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3));
// #[rustfmt::skip]
// make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4));
// #[rustfmt::skip]
// make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5));
// #[rustfmt::skip]
// make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6));
// #[rustfmt::skip]
// make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7));

// TODO: closured with more that 8 arguments are not supported. Bother with a workaround?

// #[rustfmt::skip]
// make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8));
// #[rustfmt::skip]
// make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8), (p9, P9));
// #[rustfmt::skip]
// make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8), (p9, P9), (p10, P10));
// #[rustfmt::skip]
// make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8), (p9, P9), (p10, P10), (p11, P11));
