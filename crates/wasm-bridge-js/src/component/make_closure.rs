use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::*, JsValue};

use crate::{DataHandle, DropHandler, FromJsValue, Result, StoreContextMut};

pub(crate) type MakeClosure<T> = Box<dyn Fn(DataHandle<T>) -> (JsValue, DropHandler)>;

pub trait IntoMakeClosure<T, Params, Results> {
    fn into_make_closure(self) -> MakeClosure<T>;
}

impl<T, Params, Results, F> IntoMakeClosure<T, Params, Results> for F
where
    T: 'static,
    Params: FromJsValue + 'static,
    Results: Into<JsValue> + 'static,
    F: Fn(StoreContextMut<T>, (Params,)) -> Result<(Results,)> + 'static,
{
    fn into_make_closure(self) -> MakeClosure<T> {
        let self_rc = Rc::new(self);

        let make_closure = move |handle: DataHandle<T>| {
            let self_clone = self_rc.clone();
            // let handle_clone = ha

            let closure = Closure::<dyn Fn(JsValue) -> JsValue>::new(move |arg| {
                // TODO: change unwraps to user errors
                let arg = (Params::from_js_value(&arg).unwrap(),);
                let results = self_clone(&mut handle.try_lock().unwrap(), arg).unwrap();
                results.0.into()
            });

            let js_val: JsValue = closure.as_ref().into();

            (js_val, DropHandler::new(closure))
        };

        Box::new(make_closure)
    }
}

// macro_rules! make_closure {
//     ($(($param: ident, $name: ident)),*) => {
//         impl<T, $($name, )* R, F> IntoMakeClosure<T, ($($name,)*),  R> for F
//         where
//             T: 'static,
//             $($name: FromWasmAbi + 'static,)*
//             R: IntoImportResults + 'static,
//             F: Fn(Caller<T>, $($name),*) -> R + 'static,
//         {
//             fn into_make_closure(self) -> MakeClosure<T> {
//                 let self_rc = Rc::new(self);

//                 let make_closure = move |handle: DataHandle<T>| {
//                     let caller = Caller::new(handle);
//                     let self_clone = self_rc.clone();

//                     let closure =
//                         Closure::<dyn Fn($($name),*) -> R::Results + 'static>::new(move |$($param: $name),*| {
//                             self_clone(caller.clone(), $($param),*).into_import_results()
//                         });

//                     let js_val: JsValue = closure.as_ref().into();

//                     (js_val, DropHandler::new(closure))
//                 };

//                 Box::new(make_closure)
//             }
//         }
//     };
// }

// #[rustfmt::skip]
// make_closure!();
// #[rustfmt::skip]
// make_closure!((p0, P0));
// #[rustfmt::skip]
// make_closure!((p0, P0), (p1, P1));
// #[rustfmt::skip]
// make_closure!((p0, P0), (p1, P1), (p2, P2));
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
// #[rustfmt::skip]
// make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8));
// #[rustfmt::skip]
// make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8), (p9, P9));
// #[rustfmt::skip]
// make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8), (p9, P9), (p10, P10));
// #[rustfmt::skip]
// make_closure!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7), (p8, P8), (p9, P9), (p10, P10), (p11, P11));
