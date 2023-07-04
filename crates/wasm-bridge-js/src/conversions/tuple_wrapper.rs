use js_sys::{Array, Reflect};
use wasm_bindgen::{
    convert::{IntoWasmAbi, ReturnWasmAbi, WasmAbi},
    describe::WasmDescribe,
    JsValue,
};

macro_rules! tuple_wrapper {
    ($wrapper: ident, $count: literal, $(($index: tt, $name: ident)),*) => {
        pub struct $wrapper<$($name),*>($(pub $name),*);

        impl<$($name),*> WasmDescribe for $wrapper<$($name),*> {
            fn describe() {
                JsValue::describe()
            }
        }

        unsafe impl<$($name: WasmAbi),*> WasmAbi for $wrapper<$($name),*> {}

        impl<$($name: Into<JsValue>),*> ReturnWasmAbi for $wrapper<$($name),*> {
            type Abi = <JsValue as IntoWasmAbi>::Abi;

            fn return_abi(self) -> Self::Abi {
                let result: JsValue = Array::new_with_length($count).into();
                $(Reflect::set_u32(&result, $index, &self.$index.into()).unwrap();)*
                result.into_abi()
            }
        }
    };
}

#[rustfmt::skip]
tuple_wrapper!(TupleWrapper2,   2, (0, T0), (1, T1));
#[rustfmt::skip]
tuple_wrapper!(TupleWrapper3,   3, (0, T0), (1, T1), (2, T2));
#[rustfmt::skip]
tuple_wrapper!(TupleWrapper4,   4, (0, T0), (1, T1), (2, T2), (3, T3));
#[rustfmt::skip]
tuple_wrapper!(TupleWrapper5,   5, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
#[rustfmt::skip]
tuple_wrapper!(TupleWrapper6,   6, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
#[rustfmt::skip]
tuple_wrapper!(TupleWrapper7,   7, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6));
#[rustfmt::skip]
tuple_wrapper!(TupleWrapper8,   8, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7));
#[rustfmt::skip]
tuple_wrapper!(TupleWrapper9,   9, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8));
#[rustfmt::skip]
tuple_wrapper!(TupleWrapper10, 10, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9));
#[rustfmt::skip]
tuple_wrapper!(TupleWrapper11, 11, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9), (10, T10));
#[rustfmt::skip]
tuple_wrapper!(TupleWrapper12, 12, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9), (10, T10), (11, T11));
