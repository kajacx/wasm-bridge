use js_sys::{Array, Int32Array, Reflect};
use wasm_bindgen::{convert::ReturnWasmAbi, JsValue};

pub trait IntoJsValue {
    type ReturnAbi: ReturnWasmAbi;

    fn into_js_value(self) -> JsValue;

    /// When this is returned from a closure
    fn into_return_abi(self) -> Self::ReturnAbi;

    /// Number of function arguments when this type is used as a function input type
    fn number_of_args() -> u32;

    /// Convert to function arguments when calling a function with this value
    fn into_function_args(self) -> Array;
}

impl IntoJsValue for () {
    type ReturnAbi = Self;

    fn into_js_value(self) -> JsValue {
        JsValue::undefined()
    }

    fn into_return_abi(self) -> Self::ReturnAbi {
        self
    }

    fn number_of_args() -> u32 {
        0
    }

    fn into_function_args(self) -> Array {
        Array::new()
    }
}

impl<'a> IntoJsValue for &'a str {
    type ReturnAbi = Self;

    fn into_js_value(self) -> JsValue {
        self.into()
    }

    fn into_return_abi(self) -> Self::ReturnAbi {
        self
    }

    fn number_of_args() -> u32 {
        1
    }

    fn into_function_args(self) -> Array {
        Array::of1(&self.into_js_value())
    }
}

macro_rules! into_js_value_single {
    ($ty: ty) => {
        impl IntoJsValue for $ty {
            type ReturnAbi = Self;

            fn into_js_value(self) -> JsValue {
                self.into()
            }

            fn into_return_abi(self) -> Self::ReturnAbi {
                self
            }

            fn number_of_args() -> u32 {
                1
            }

            fn into_function_args(self) -> Array {
                Array::of1(&self.into_js_value())
            }
        }
    };
}

into_js_value_single!(i32);
into_js_value_single!(i64);
into_js_value_single!(u32);
into_js_value_single!(u64);
into_js_value_single!(f32);
into_js_value_single!(f64);
into_js_value_single!(String);

impl<T: IntoJsValue> IntoJsValue for Option<T> {
    // TODO: should be able to return Option ... ?
    // type ReturnAbi = OptionIntoWasmAbi<T::ReturnAbi>;
    type ReturnAbi = JsValue;

    fn into_js_value(self) -> JsValue {
        match self {
            Self::Some(value) => value.into_js_value(),
            None => JsValue::undefined(),
        }
    }

    fn into_return_abi(self) -> Self::ReturnAbi {
        match self {
            Self::Some(value) => value.into_js_value(),
            None => JsValue::undefined(),
        }
    }

    fn number_of_args() -> u32 {
        1 // TODO: verify
    }

    fn into_function_args(self) -> Array {
        Array::of1(&self.into_js_value())
    }
}

// FIXME: Copy bound is bad
impl<'a, T: IntoJsValue + Copy> IntoJsValue for &'a [T] {
    type ReturnAbi = JsValue;

    fn into_js_value(self) -> JsValue {
        let array = Int32Array::new_with_length(self.len() as _);
        self.into_iter().enumerate().for_each(|(index, item)| {
            // TODO: set_index is probably faster to Int32Array and "friends"
            Reflect::set_u32(&array, index as _, &item.into_js_value()).expect("array is array");
        });
        array.into()
    }

    fn into_return_abi(self) -> Self::ReturnAbi {
        self.into_js_value()
    }

    fn number_of_args() -> u32 {
        1
    }

    fn into_function_args(self) -> Array {
        Array::of1(&self.into_js_value())
    }
}

impl<T: IntoJsValue + Copy> IntoJsValue for Vec<T> {
    type ReturnAbi = JsValue;

    fn into_js_value(self) -> JsValue {
        let as_slice: &[T] = &self;
        as_slice.into_js_value()
    }

    fn into_return_abi(self) -> Self::ReturnAbi {
        self.into_js_value()
    }

    fn number_of_args() -> u32 {
        1
    }

    fn into_function_args(self) -> Array {
        Array::of1(&self.into_js_value())
    }
}

impl<T: IntoJsValue> IntoJsValue for (T,) {
    type ReturnAbi = T::ReturnAbi;

    fn into_js_value(self) -> JsValue {
        self.0.into_js_value()
    }

    fn into_return_abi(self) -> Self::ReturnAbi {
        self.0.into_return_abi()
    }

    fn number_of_args() -> u32 {
        T::number_of_args()
    }

    fn into_function_args(self) -> Array {
        self.0.into_function_args()
    }
}

macro_rules! into_js_value_many {
    ($count: literal, $(($index: tt, $name: ident)),*) => {
        impl<$($name: IntoJsValue),*> IntoJsValue for ($($name, )*) {
            type ReturnAbi = JsValue;

            fn into_js_value(self) -> JsValue {
                self.into_function_args().into()
            }

            fn into_return_abi(self) -> Self::ReturnAbi {
                self.into_js_value()
            }

            fn number_of_args() -> u32 {
                $count
            }

            fn into_function_args(self) -> Array {
                // TODO: test is "ofN" is faster, and by how much
                let result = Array::new_with_length($count);
                $( Reflect::set_u32(&result, $index, &self.$index.into_js_value()).expect("result is array"); )*
                result
            }
        }
    };
}

#[rustfmt::skip]
into_js_value_many!( 2, (0, T0), (1, T1));
#[rustfmt::skip]
into_js_value_many!( 3, (0, T0), (1, T1), (2, T2));
#[rustfmt::skip]
into_js_value_many!( 4, (0, T0), (1, T1), (2, T2), (3, T3));
#[rustfmt::skip]
into_js_value_many!( 5, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
#[rustfmt::skip]
into_js_value_many!( 6, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
#[rustfmt::skip]
into_js_value_many!( 7, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6));
#[rustfmt::skip]
into_js_value_many!( 8, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7));
#[rustfmt::skip]
into_js_value_many!( 9, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8));
#[rustfmt::skip]
into_js_value_many!(10, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9));
#[rustfmt::skip]
into_js_value_many!(11, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9), (10, T10));
#[rustfmt::skip]
into_js_value_many!(12, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9), (10, T10), (11, T11));
