use js_sys::Array;
use wasm_bindgen::{
    convert::{FromWasmAbi, IntoWasmAbi, ReturnWasmAbi, WasmAbi},
    describe::WasmDescribe,
    prelude::Closure,
    JsValue,
};

use crate::*;

pub trait IntoClosure<Params, Results> {
    fn into_closure(self) -> (JsValue, DropHandler);
}

// impl<P, R, F> IntoClosure<P, R> for F
// where
//     F: Fn(Caller<()>, P) -> R + 'static,
//     P: FromWasmAbi + 'static,
//     R: ReturnWasmAbi + 'static,
// {
//     fn into_closure(self) -> (JsValue, DropHandler) {
//         let closure =
//             Closure::<dyn Fn(P) -> R + 'static>::new(move |arg: P| self(Caller::new(), arg));

//         let js_val: JsValue = closure.as_ref().into();

//         (js_val, DropHandler::new(closure))
//     }
// }

macro_rules! impl_into_closure_single {
    ($ty:ty) => {
        impl<R, F> IntoClosure<$ty, R> for F
        where
            F: Fn(Caller<()>, $ty) -> R + 'static,
            R: ReturnWasmAbi + 'static,
        {
            fn into_closure(self) -> (JsValue, DropHandler) {
                let closure = Closure::<dyn Fn($ty) -> R + 'static>::new(move |arg: $ty| {
                    self(Caller::new(), arg)
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

impl<P0, P1, R, F> IntoClosure<(P0, P1), R> for F
where
    F: Fn(Caller<()>, P0, P1) -> R + 'static,
    P0: FromWasmAbi + 'static,
    P1: FromWasmAbi + 'static,
    R: ReturnWasmAbi + 'static,
{
    fn into_closure(self) -> (JsValue, DropHandler) {
        let closure = Closure::<dyn Fn(P0, P1) -> R + 'static>::new(move |arg0: P0, arg1: P1| {
            self(Caller::new(), arg0, arg1)
        });

        let js_val: JsValue = closure.as_ref().into();

        (js_val, DropHandler::new(closure))
    }
}

pub struct MyPair(pub i32, pub i32);

impl WasmDescribe for MyPair {
    fn describe() {
        // panic!("I am being described :)")
        // inform(wasm_bindgen::describe::EXTERNREF)
        JsValue::describe();
    }
}

unsafe impl WasmAbi for MyPair {}

impl ReturnWasmAbi for MyPair {
    type Abi = <JsValue as IntoWasmAbi>::Abi;

    fn return_abi(self) -> Self::Abi {
        let result: JsValue = Array::of2(&self.0.into(), &self.1.into()).into();
        result.into_abi()
    }
}
