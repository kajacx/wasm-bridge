use wasm_bindgen::{
    convert::{FromWasmAbi, ReturnWasmAbi},
    prelude::Closure,
    JsValue,
};

use crate::*;

pub trait IntoClosure<T: ?Sized> {
    fn into_closure(self) -> (JsValue, DropHandler);
}

impl<P, R, F> IntoClosure<dyn Fn(P) -> R> for F
where
    F: Fn(Caller<()>, P) -> R + 'static,
    P: FromWasmAbi + 'static,
    R: ReturnWasmAbi + 'static,
{
    fn into_closure(self) -> (JsValue, DropHandler) {
        let closure =
            Closure::<dyn Fn(P) -> R + 'static>::new(move |arg: P| self(Caller::new(), arg));

        let js_val: JsValue = closure.as_ref().into();

        (js_val, DropHandler::new(closure))
    }
}
