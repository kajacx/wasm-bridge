use js_sys::{Array, Reflect};
use wasm_bindgen::{
    convert::{IntoWasmAbi, ReturnWasmAbi, WasmAbi},
    describe::WasmDescribe,
    JsValue,
};

pub struct TupleWrapper2<T0, T1>(T0, T1);

impl<T0, T1> WasmDescribe for TupleWrapper2<T0, T1> {
    fn describe() {
        JsValue::describe()
    }
}

unsafe impl<T0: WasmAbi, T1: WasmAbi> WasmAbi for TupleWrapper2<T0, T1> {}

impl<T0: Into<JsValue>, T1: Into<JsValue>> ReturnWasmAbi for TupleWrapper2<T0, T1> {
    type Abi = <JsValue as IntoWasmAbi>::Abi;

    fn return_abi(self) -> Self::Abi {
        // let result: JsValue = Array::of2(&self.0.into(), &self.1.into()).into();
        let result: JsValue = Array::new_with_length(2).into();
        Reflect::set_u32(&result, 0, &self.0.into()).unwrap();
        Reflect::set_u32(&result, 1, &self.1.into()).unwrap();
        result.into_abi()
    }
}
