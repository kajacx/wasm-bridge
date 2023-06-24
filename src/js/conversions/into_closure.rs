use wasm_bindgen::{convert::FromWasmAbi, prelude::Closure, JsValue};

use crate::*;

pub trait IntoClosure<Params, Results> {
    fn into_closure(self) -> (JsValue, DropHandler);
}

macro_rules! impl_into_closure_single {
    ($ty:ty) => {
        impl<R, F> IntoClosure<$ty, R> for F
        where
            F: Fn(Caller<()>, $ty) -> R + 'static,
            R: IntoImportResults + 'static,
        {
            fn into_closure(self) -> (JsValue, DropHandler) {
                let closure =
                    Closure::<dyn Fn($ty) -> R::Results + 'static>::new(move |arg: $ty| {
                        self(Caller::new(), arg).into_import_results()
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

// impl<P0, P1, R, F> IntoClosure<(P0, P1), R> for F
// where
//     F: Fn(Caller<()>, P0, P1) -> R + 'static,
//     P0: FromWasmAbi + 'static,
//     P1: FromWasmAbi + 'static,
//     R: IntoImportResults + 'static,
// {
//     fn into_closure(self) -> (JsValue, DropHandler) {
//         let closure =
//             Closure::<dyn Fn(P0, P1) -> R::Results + 'static>::new(move |arg0: P0, arg1: P1| {
//                 self(Caller::new(), arg0, arg1).into_import_results()
//             });

//         let js_val: JsValue = closure.as_ref().into();

//         (js_val, DropHandler::new(closure))
//     }
// }

macro_rules! into_closure_many {
    ($(($param: ident, $name: ident)),*) => {
        impl<$($name, )* R, F> IntoClosure<($($name),*), R> for F
        where
            F: Fn(Caller<()> $(, $name)*) -> R + 'static,
            $($name: FromWasmAbi + 'static,)*
            R: IntoImportResults + 'static,
        {
            fn into_closure(self) -> (JsValue, DropHandler) {
                let closure =
                    Closure::<dyn Fn($($name),*) -> R::Results + 'static>::new(move |$($param: $name),*| {
                        self(Caller::new() $(, $param)*).into_import_results()
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

// #[rustfmt::skip]
// into_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8));
// #[rustfmt::skip]
// into_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8), (p9, P9));
// #[rustfmt::skip]
// into_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8), (p9, P9), (p10, P10));
// #[rustfmt::skip]
// into_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8), (p9, P9), (p10, P10), (p11, P11));
