use std::rc::Rc;

use wasm_bindgen::{
    convert::{FromWasmAbi, IntoWasmAbi, ReturnWasmAbi},
    prelude::Closure,
    JsValue,
};

use crate::*;

pub(crate) type MakeClosure<T> = Box<dyn Fn(DataHandle<T>) -> (JsValue, DropHandle)>;

pub trait IntoMakeClosure<T, Params, Results> {
    fn into_make_closure(self) -> MakeClosure<T>;
}

fn main() {
    let closure = Closure::<dyn Fn(i32) -> i32>::new(|i| i + 5);
    let closure = Closure::<dyn Fn(i32) -> Result<i32, JsValue>>::new(|i| Ok(i + 5));

    make_closures::<i32>();
    make_closures_fixed::<i32>();
}

fn make_closures<T: IntoWasmAbi + 'static>() {
    let closure = Closure::<dyn Fn(i32) -> T>::new(|_| todo!());
    let closure = Closure::<dyn Fn(i32) -> Result<T, JsValue>>::new(|_| todo!());
}

// But this is where it gets weird
fn make_closures_fixed<T: IntoWasmAbi + 'static>()
where
    Result<T, JsValue>: ReturnWasmAbi,
{
    let closure = Closure::<dyn Fn(i32) -> T>::new(|_| todo!()); //works
    let closure = Closure::<dyn Fn(i32) -> Result<T, JsValue>>::new(|_| todo!());
    // this now works
}

fn accept_return_wasm_abi<T: ReturnWasmAbi>() {}

fn test_it_i32() {
    accept_return_wasm_abi::<i32>(); // works
    accept_return_wasm_abi::<Result<i32, JsValue>>(); // works
}

fn test_it_into_wasm_abi<T: IntoWasmAbi + 'static>() {
    accept_return_wasm_abi::<T>(); // works
    accept_return_wasm_abi::<Result<T, JsValue>>();
    // again, works in minimal reproducible example, but not in main project
}

fn test_it_into_wasm_abi_fixed<T: IntoWasmAbi + 'static>()
where
    Result<T, JsValue>: ReturnWasmAbi, // <-- added this trait bound
{
    accept_return_wasm_abi::<T>(); // works
    accept_return_wasm_abi::<Result<T, JsValue>>();
}

impl<T, R, F> IntoMakeClosure<T, (), R> for F
where
    T: 'static,
    F: Fn(Caller<T>) -> R + 'static,
    R: ToJsValue + 'static,
    // Result<R::ReturnAbi, JsValue>: ReturnWasmAbi, // TODO: unnecessary return bound?
{
    fn into_make_closure(self) -> MakeClosure<T> {
        let self_rc = Rc::new(self);

        let make_closure = move |handle: DataHandle<T>| {
            let caller = Caller::new(handle);
            let self_clone = self_rc.clone();

            let closure = Closure::<dyn Fn() -> Result<R::ReturnAbi, JsValue>>::new(move || {
                self_clone(caller.clone()).into_return_abi()
            });

            DropHandle::from_closure(closure)
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
            Result<R::ReturnAbi, JsValue>: ReturnWasmAbi, // TODO: unnecessary return bound?
        {
            fn into_make_closure(self) -> MakeClosure<T> {
                let self_rc = Rc::new(self);

                let make_closure = move |handle: DataHandle<T>| {
                    let caller = Caller::new(handle);
                    let self_clone = self_rc.clone();

                    let closure = Closure::<dyn Fn($ty) -> Result<R::ReturnAbi, JsValue>>::new(
                        move |arg: $ty| self_clone(caller.clone(), arg).into_return_abi(),
                    );

                    DropHandle::from_closure(closure)
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

impl<T, P0, P1, R, F> IntoMakeClosure<T, (P0, P1), R> for F
where
    T: 'static,
    F: Fn(Caller<T>, P0, P1) -> R + 'static,
    P0: FromWasmAbi + 'static,
    P1: FromWasmAbi + 'static,
    R: ToJsValue + 'static,
    Result<R::ReturnAbi, JsValue>: ReturnWasmAbi, // TODO: unnecessary return bound?
{
    fn into_make_closure(self) -> MakeClosure<T> {
        let self_rc = Rc::new(self);

        let make_closure = move |handle: DataHandle<T>| {
            let caller = Caller::new(handle);
            let self_clone = self_rc.clone();

            let closure = Closure::<dyn Fn(P0, P1) -> Result<R::ReturnAbi, JsValue>>::new(
                move |p0: P0, p1: P1| self_clone(caller.clone(), p0, p1).into_return_abi(),
            );

            DropHandle::from_closure(closure)
        };

        Box::new(make_closure)
    }
}

// macro_rules! into_make_closure_many {
//     ($(($param: ident, $name: ident)),*) => {
//         impl<T, $($name, )* R, F> IntoMakeClosure<T, ($($name),*), R> for F
//         where
//             T: 'static,
//             F: Fn(Caller<T>, $($name),*) -> R + 'static,
//             $($name: FromWasmAbi + 'static,)*
//             R: ToJsValue + 'static,
//         {
//             fn into_make_closure(self) -> MakeClosure<T> {
//                 let self_rc = Rc::new(self);

//                 let make_closure = move |handle: DataHandle<T>| {
//                     let caller = Caller::new(handle);
//                     let self_clone = self_rc.clone();

//                     let closure =
//                         Closure::<dyn Fn($($name),*) -> Result<R::ReturnAbi, JsValue>>::new(move |$($param: $name),*| {
//                             self_clone(caller.clone(), $($param),*).into_return_abi()
//                         });

//                     DropHandle::from_closure(closure)
//                 };

//                 Box::new(make_closure)
//             }
//         }
//     };
// }

// // #[rustfmt::skip]
// // into_make_closure_many!((p0, P0), (p1, P1));
// #[rustfmt::skip]
// into_make_closure_many!((p0, P0), (p1, P1), (p2, P2));
// #[rustfmt::skip]
// into_make_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3));
// #[rustfmt::skip]
// into_make_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4));
// #[rustfmt::skip]
// into_make_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5));
// #[rustfmt::skip]
// into_make_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6));
// #[rustfmt::skip]
// into_make_closure_many!((p0, P0), (p1, P1), (p2, P2), (p3, P3), (p4, P4), (p5, P5), (p6, P6), (p7, P7));

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
