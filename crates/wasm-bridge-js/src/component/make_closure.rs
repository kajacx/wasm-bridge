use std::rc::Rc;

use wasm_bindgen::{convert::FromWasmAbi, prelude::*, JsValue};

use crate::{DataHandle, DropHandler, IntoJsValue, Result, StoreContextMut};

pub(crate) type MakeClosure<T> = Box<dyn Fn(DataHandle<T>) -> (JsValue, DropHandler)>;

pub trait IntoMakeClosure<T, Params, Results> {
    fn into_make_closure(self) -> MakeClosure<T>;
}

macro_rules! make_closure {
    ($(($param: ident, $name: ident)),*) => {
        impl<T, $($name, )* R, F> IntoMakeClosure<T, ($($name,)*), R> for F
        where
            T: 'static,
            $($name: FromWasmAbi + 'static,)*
            R: IntoJsValue + 'static,
            F: Fn(StoreContextMut<T>, ($($name, )*)) -> Result<R> + 'static,
        {
            fn into_make_closure(self) -> MakeClosure<T> {
                let self_rc = Rc::new(self);

                let make_closure = move |handle: DataHandle<T>| {
                    let self_clone = self_rc.clone();

                    let closure =
                        Closure::<dyn Fn($($name),*) -> R::ReturnAbi>::new(move |$($param: $name),*| {
                            // TODO: user error?
                            self_clone(&mut handle.borrow_mut(), ($($param,)*)).unwrap().into_return_abi()
                        });

                    DropHandler::from_closure(closure)
                };

                Box::new(make_closure)
            }
        }
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
