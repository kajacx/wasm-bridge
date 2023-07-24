use std::rc::Rc;

use wasm_bindgen::{convert::FromWasmAbi, prelude::Closure, JsValue};

use crate::*;

pub(crate) type MakeClosure<T> = Box<dyn Fn(DataHandle<T>) -> (JsValue, DropHandler)>;

pub trait IntoMakeClosure<T, Params, Results> {
    fn into_make_closure(self, engine: Engine) -> MakeClosure<T>;
}

impl<T, R, F> IntoMakeClosure<T, (), R> for F
where
    T: 'static,
    F: Fn(Caller<T>) -> R + 'static,
    R: ToJsValue + 'static,
{
    fn into_make_closure(self, engine: Engine) -> MakeClosure<T> {
        let self_rc = Rc::new(self);

        let make_closure = move |handle: DataHandle<T>| {
            let caller = Caller::new(Store {
                engine: engine.clone(),
                data: handle,
            });
            let self_clone = self_rc.clone();

            let closure = Closure::<dyn Fn() -> Result<R::ReturnAbi, JsValue>>::new(move || {
                self_clone(caller.clone()).into_return_abi()
            });

            DropHandler::from_closure(closure)
        };

        Box::new(make_closure)
    }
}

macro_rules! into_make_closure_single {
    ($ty:ty) => {
        impl<T, R, F> IntoMakeClosure<T, $ty, R> for F
        where
            T: 'static,
            F: Fn(Caller<T>, $ty) -> R + 'static,
            R: ToJsValue + 'static,
        {
            fn into_make_closure(self, engine: Engine) -> MakeClosure<T> {
                let self_rc = Rc::new(self);

                let make_closure = move |handle: DataHandle<T>| {
                    let caller = Caller::new(Store {
                        engine: engine.clone(),
                        data: handle,
                    });
                    let self_clone = self_rc.clone();

                    let closure = Closure::<dyn Fn($ty) -> Result<R::ReturnAbi, JsValue>>::new(
                        move |arg: $ty| self_clone(caller.clone(), arg).into_return_abi(),
                    );

                    DropHandler::from_closure(closure)
                };

                Box::new(make_closure)
            }
        }
    };
}

into_make_closure_single!(i32);
into_make_closure_single!(i64);
into_make_closure_single!(u32);
into_make_closure_single!(u64);
into_make_closure_single!(f32);
into_make_closure_single!(f64);

macro_rules! into_make_closure_many {
    ($(($param: ident, $name: ident)),*) => {
        impl<T, $($name, )* R, F> IntoMakeClosure<T, ($($name),*), R> for F
        where
            T: 'static,
            F: Fn(Caller<T>, $($name),*) -> R + 'static,
            $($name: FromWasmAbi + 'static,)*
            R: ToJsValue + 'static,
        {
            fn into_make_closure(self, engine: Engine) -> MakeClosure<T> {
                let self_rc = Rc::new(self);

                let make_closure = move |handle: DataHandle<T>| {
                    let caller = Caller::new(Store { engine: engine.clone(), data: handle });
                    let self_clone = self_rc.clone();

                    let closure =
                        Closure::<dyn Fn($($name),*) -> Result<R::ReturnAbi, JsValue>>::new(move |$($param: $name),*| {
                            self_clone(caller.clone(), $($param),*).into_return_abi()
                        });

                    DropHandler::from_closure(closure)
                };

                Box::new(make_closure)
            }
        }
    };
}

#[rustfmt::skip]
into_make_closure_many!((p0, P0), (p1, P1));
#[rustfmt::skip]
into_make_closure_many!((p0, P0), (p1, P1), (p2, P2));
#[rustfmt::skip]
into_make_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3));
#[rustfmt::skip]
into_make_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4));
#[rustfmt::skip]
into_make_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5));
#[rustfmt::skip]
into_make_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6));
#[rustfmt::skip]
into_make_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7));

// js-sys doesn't support closures with more than 8 arguments
// TODO: a workaround can exist though

// #[rustfmt::skip]
// into_make_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8));
// #[rustfmt::skip]
// into_make_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8), (p9, P9));
// #[rustfmt::skip]
// into_make_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8), (p9, P9), (p10, P10));
// #[rustfmt::skip]
// into_make_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8), (p9, P9), (p10, P10), (p11, P11));
