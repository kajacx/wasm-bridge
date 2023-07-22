// use std::future::Future;
use std::rc::Rc;

use wasm_bindgen::{prelude::*, JsValue};

use crate::{DataHandle, DropHandler, FromJsValue, Result, StoreContextMut, ToJsValue};

pub(crate) type MakeClosure<T> = Box<dyn Fn(DataHandle<T>) -> (JsValue, DropHandler)>;

pub trait IntoMakeClosure<T, Params, Results> {
    fn into_make_closure(self) -> MakeClosure<T>;
}

macro_rules! make_closure {
    ($(($param: ident, $name: ident)),*) => {
        impl<T, $($name, )* R, F> IntoMakeClosure<T, ($($name,)*), R> for F
        where
            T: 'static,
            $($name: FromJsValue + 'static,)*
            R: ToJsValue + 'static ,
            F: Fn(StoreContextMut<T>, ($($name, )*)) -> Result<R> + 'static,
        {
            fn into_make_closure(self) -> MakeClosure<T> {
                let self_rc = Rc::new(self);

                let make_closure = move |handle: DataHandle<T>| {
                    let self_clone = self_rc.clone();

                    let closure =
                        Closure::<dyn Fn($($name::WasmAbi),*) -> Result<R::ReturnAbi, JsValue>>::new(move |$($param: $name::WasmAbi),*| {
                            self_clone(
                                &mut handle.borrow_mut(),
                                ($($name::from_wasm_abi($param).map_err::<JsValue, _>(|err| format!("import value from abi error: {err:?}").into())?,)*)
                            ).map_err(|err| format!("host imported fn returned error: {err:?}"))?.into_return_abi()
                        });

                    DropHandler::from_closure(closure)
                };

                Box::new(make_closure)
            }
        }

        // impl<T, $($name, )* R, F, Fut> IntoMakeClosure<T, ($($name,)*), R> for F
        // where
        //     T: 'static,
        //     $($name: FromJsValue + 'static,)*
        //     R: ToJsValue + 'static ,
        //     F: Fn(StoreContextMut<T>, ($($name, )*)) -> Fut + 'static,
        //     Fut: Future<Output = Result<R>>
        // {
        //     fn into_make_closure(self) -> MakeClosure<T> {
        //         let self_rc = Rc::new(self);

        //         let make_closure = move |handle: DataHandle<T>| {
        //             let self_clone = self_rc.clone();

        //             let closure =
        //                 Closure::<dyn Fn($($name::WasmAbi),*) -> Result<R::ReturnAbi, JsValue>>::new(move |$($param: $name::WasmAbi),*| {
        //                     self_clone(
        //                         &mut handle.borrow_mut(),
        //                         ($($name::from_wasm_abi($param).map_err::<JsValue, _>(|err| format!("import value from abi error: {err:?}").into())?,)*)
        //                     ).map_err(|err| format!("host imported fn returned error: {err:?}"))?.into_return_abi()
        //                 });

        //             DropHandler::from_closure(closure)
        //         };

        //         Box::new(make_closure)
        //     }
        // }
    };
}

#[rustfmt::skip]
make_closure!();
#[rustfmt::skip]
make_closure!((p0, P0));
#[rustfmt::skip]
make_closure!((p0, P0), (p1, P1));
#[rustfmt::skip]
make_closure!((p0, P0), (p1, P1), (p2, P2));
#[rustfmt::skip]
make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3));
#[rustfmt::skip]
make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4));
#[rustfmt::skip]
make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5));
#[rustfmt::skip]
make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6));
#[rustfmt::skip]
make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7));

// TODO: closured with more that 8 arguments are not supported. Bother with a workaround?

// #[rustfmt::skip]
// make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8));
// #[rustfmt::skip]
// make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8), (p9, P9));
// #[rustfmt::skip]
// make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8), (p9, P9), (p10, P10));
// #[rustfmt::skip]
// make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8), (p9, P9), (p10, P10), (p11, P11));
