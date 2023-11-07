// use std::future::Future;
use std::rc::Rc;

use wasm_bindgen::{prelude::*, JsValue};

use crate::{
    component::{Lift, Lower, LowerContext},
    DataHandle, DropHandle, Result, StoreContextMut,
};

pub(crate) type MakeClosure<T> = Box<dyn Fn(DataHandle<T>) -> (JsValue, DropHandle)>;

/// Converts a Rust function into a JavaScript closure which automatically converts parameters and
/// return values to/from javascript using `Lift` and `Lower`
pub trait IntoMakeClosure<T, Params, Results> {
    fn into_make_closure(self) -> MakeClosure<T>;
}

macro_rules! as_ret {
    ($ty: ty) => {
        JsValue
    };
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

                let make_closure = move |handle: DataHandle<T>| {
                    let self_clone = self_rc.clone();

                    let closure =
                        Closure::<dyn Fn($(as_ret!($name)),*) -> Result<JsValue, JsValue>>::new(move |$($param: JsValue),*| {
                            let params = ($($name::lift(&$param),)*);

                            let ret = self_clone(
                                &mut handle.borrow_mut(),
                                params,
                            ).map_err(|err| format!("host imported fn returned error: {err:?}"))?;

                            let cx = LowerContext {};

                            let ret = ret.lower_ret(&cx);

                            Ok(ret)
                        });

                    DropHandle::from_closure(closure)
                };

                Box::new(make_closure)
            }
        }
    };
}

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
