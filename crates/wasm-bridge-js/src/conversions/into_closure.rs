use wasm_bindgen::{convert::FromWasmAbi, prelude::Closure, JsValue};

use crate::*;

pub trait IntoClosure<T, Params, Results> {
    fn into_closure(self, handle: DataHandle<T>) -> (JsValue, DropHandler);
}

impl<T, R, F> IntoClosure<T, (), R> for F
where
    F: Fn(Caller<T>) -> R + 'static,
    R: IntoImportResults + 'static,
{
    fn into_closure(self, handle: DataHandle<T>) -> (JsValue, DropHandler) {
        let caller = Caller::new(handle);

        let closure = Closure::<dyn Fn() -> R::Results + 'static>::new(move || {
            self(caller.clone()).into_import_results()
        });

        let js_val: JsValue = closure.as_ref().into();

        (js_val, DropHandler::new(closure))
    }
}

macro_rules! impl_into_closure_single {
    ($ty:ty) => {
        impl<T, R, F> IntoClosure<T, $ty, R> for F
        where
            // T: 'static, TODO: why is this not required?
            F: Fn(Caller<T>, $ty) -> R + 'static,
            R: IntoImportResults + 'static,
        {
            fn into_closure(self, handle: DataHandle<T>) -> (JsValue, DropHandler) {
                let caller = Caller::new(handle);

                let closure =
                    Closure::<dyn Fn($ty) -> R::Results + 'static>::new(move |arg: $ty| {
                        self(caller.clone(), arg).into_import_results()
                    });

                let js_val: JsValue = closure.as_ref().into();

                (js_val, DropHandler::new(closure))
            }
        }
    };
}

impl_into_closure_single!(i32);
impl_into_closure_single!(i64);
impl_into_closure_single!(u32);
impl_into_closure_single!(u64);
impl_into_closure_single!(f32);
impl_into_closure_single!(f64);

macro_rules! into_closure_many {
    ($(($param: ident, $name: ident)),*) => {
        impl<T, $($name, )* R, F> IntoClosure<T, ($($name),*), R> for F
        where
            F: Fn(Caller<T>, $($name),*) -> R + 'static,
            $($name: FromWasmAbi + 'static,)*
            R: IntoImportResults + 'static,
        {
            fn into_closure(self, handle: DataHandle<T>) -> (JsValue, DropHandler) {
                let caller = Caller::new(handle);

                let closure =
                    Closure::<dyn Fn($($name),*) -> R::Results + 'static>::new(move |$($param: $name),*| {
                        self(caller.clone(), $($param),*).into_import_results()
                    });

                let js_val: JsValue = closure.as_ref().into();

                (js_val, DropHandler::new(closure))
            }
        }
    };
}

#[rustfmt::skip]
into_closure_many!((p0, P0), (p1, P1));
#[rustfmt::skip]
into_closure_many!((p0, P0), (p1, P1), (p2, P2));
#[rustfmt::skip]
into_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3));
#[rustfmt::skip]
into_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4));
#[rustfmt::skip]
into_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5));
#[rustfmt::skip]
into_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6));
#[rustfmt::skip]
into_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7));

// js-sys doesn't support closured with more than 7 arguments
// TODO: a workaround can exist though

// #[rustfmt::skip]
// into_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8));
// #[rustfmt::skip]
// into_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8), (p9, P9));
// #[rustfmt::skip]
// into_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8), (p9, P9), (p10, P10));
// #[rustfmt::skip]
// into_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8), (p9, P9), (p10, P10), (p11, P11));
