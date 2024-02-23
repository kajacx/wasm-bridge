use js_sys::Array;
use wasm_bindgen::{
    convert::{IntoWasmAbi, ReturnWasmAbi},
    JsValue,
};

use crate::Val;

pub trait ToJsValue: Sized {
    type ReturnAbi: ReturnWasmAbi + IntoWasmAbi;

    fn to_js_value(&self) -> JsValue;

    /// When this is returned from a closure
    fn into_return_abi(self) -> Result<Self::ReturnAbi, JsValue>;

    /// Number of function arguments when this type is used as a function input type
    fn number_of_args() -> u32 {
        1
    }

    /// Convert to function arguments when calling a function with this value
    fn to_function_args(&self) -> Array {
        Array::of1(&self.to_js_value())
    }
}

impl ToJsValue for () {
    type ReturnAbi = Self;

    fn to_js_value(&self) -> JsValue {
        JsValue::undefined()
    }

    fn into_return_abi(self) -> Result<Self::ReturnAbi, JsValue> {
        Ok(())
    }

    fn number_of_args() -> u32 {
        0
    }

    fn to_function_args(&self) -> Array {
        Array::new()
    }
}

macro_rules! to_js_value_single {
    ($ty: ty) => {
        impl ToJsValue for $ty {
            type ReturnAbi = Self;

            fn to_js_value(&self) -> JsValue {
                (*self).into()
            }

            fn into_return_abi(self) -> Result<Self::ReturnAbi, JsValue> {
                Ok(self)
            }
        }
    };
}

to_js_value_single!(i8);
to_js_value_single!(i16);
to_js_value_single!(i32);
to_js_value_single!(i64);

to_js_value_single!(u8);
to_js_value_single!(u16);
to_js_value_single!(u32);
to_js_value_single!(u64);

to_js_value_single!(f32);
to_js_value_single!(f64);

impl ToJsValue for Val {
    type ReturnAbi = JsValue;

    fn to_js_value(&self) -> JsValue {
        match self {
            Val::I32(val) => val.to_js_value(),
            Val::I64(val) => val.to_js_value(),
            Val::F32(bits) => f32::from_bits(*bits).to_js_value(),
            Val::F64(bits) => f64::from_bits(*bits).to_js_value(),
        }
    }

    fn into_return_abi(self) -> Result<Self::ReturnAbi, JsValue> {
        Ok(self.to_js_value())
    }
}

impl<T: ToJsValue> ToJsValue for (T,) {
    type ReturnAbi = T::ReturnAbi;

    fn to_js_value(&self) -> JsValue {
        self.0.to_js_value()
    }

    fn into_return_abi(self) -> Result<Self::ReturnAbi, JsValue> {
        self.0.into_return_abi()
    }

    fn number_of_args() -> u32 {
        T::number_of_args()
    }

    fn to_function_args(&self) -> Array {
        self.0.to_function_args()
    }
}

macro_rules! to_js_value_many {
    ($count: literal, $(($index: tt, $name: ident)),*) => {
        impl<$($name: ToJsValue),*> ToJsValue for ($($name, )*) {
            type ReturnAbi = JsValue;

            fn to_js_value(&self) -> JsValue {
                self.to_function_args().into()
            }

            fn into_return_abi(self) -> Result<Self::ReturnAbi, JsValue> {
                Ok(self.to_js_value())
            }

            fn number_of_args() -> u32 {
                $count
            }

            fn to_function_args(&self) -> Array {
                [$( &self.$index.to_js_value(), )*].iter().collect()
            }
        }
    };
}

#[rustfmt::skip]
to_js_value_many!( 2, (0, T0), (1, T1));
#[rustfmt::skip]
to_js_value_many!( 3, (0, T0), (1, T1), (2, T2));
#[rustfmt::skip]
to_js_value_many!( 4, (0, T0), (1, T1), (2, T2), (3, T3));
#[rustfmt::skip]
to_js_value_many!( 5, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
#[rustfmt::skip]
to_js_value_many!( 6, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
#[rustfmt::skip]
to_js_value_many!( 7, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6));
#[rustfmt::skip]
to_js_value_many!( 8, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7));
