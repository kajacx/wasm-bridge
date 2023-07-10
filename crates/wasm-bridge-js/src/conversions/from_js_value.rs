use js_sys::Reflect;
use wasm_bindgen::{convert::FromWasmAbi, JsValue};

use crate::*;

pub trait FromJsValue: Sized {
    type WasmAbi: FromWasmAbi;

    fn from_js_value(value: &JsValue) -> Result<Self>;

    fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self>;
}

impl FromJsValue for () {
    type WasmAbi = JsValue;

    fn from_js_value(value: &JsValue) -> Result<Self> {
        if value.is_undefined() || value.is_null() {
            Ok(())
        } else {
            Err(value.into())
        }
    }

    fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self> {
        Ok(())
    }
}

impl FromJsValue for i32 {
    type WasmAbi = Self;

    fn from_js_value(value: &JsValue) -> Result<Self> {
        match value.as_f64() {
            Some(number) => Ok(number as _),
            None => Err(value.into()), // TODO: better error, in other types too
        }
    }

    fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self> {
        Ok(abi)
    }
}

impl FromJsValue for i64 {
    type WasmAbi = Self;
    fn from_js_value(value: &JsValue) -> Result<Self> {
        Ok(value.clone().try_into()?)
    }
    fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self> {
        Ok(abi)
    }
}

impl FromJsValue for u32 {
    type WasmAbi = Self;
    fn from_js_value(value: &JsValue) -> Result<Self> {
        match value.as_f64() {
            // Conversion to i32 first needed to handle "negative" numbers
            Some(number) => Ok(number as i32 as _),
            None => Err(value.into()),
        }
    }
    fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self> {
        Ok(abi)
    }
}

impl FromJsValue for u64 {
    type WasmAbi = Self;
    fn from_js_value(value: &JsValue) -> Result<Self> {
        // Conversion to u32 first needed to handle "negative" numbers
        Ok(i64::try_from(value.clone())? as _)
    }
    fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self> {
        Ok(abi)
    }
}

impl FromJsValue for f32 {
    type WasmAbi = Self;
    fn from_js_value(value: &JsValue) -> Result<Self> {
        match value.as_f64() {
            Some(number) => Ok(number as _),
            None => Err(value.into()),
        }
    }
    fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self> {
        Ok(abi)
    }
}

impl FromJsValue for f64 {
    type WasmAbi = Self;
    fn from_js_value(value: &JsValue) -> Result<Self> {
        Ok(value.try_into()?)
    }
    fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self> {
        Ok(abi)
    }
}

impl FromJsValue for String {
    type WasmAbi = Self;

    fn from_js_value(value: &JsValue) -> Result<Self, crate::Error> {
        match value.as_string() {
            Some(value) => Ok(value),
            None => Err(value.into()), // TODO: better error
        }
    }

    fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self> {
        Ok(abi)
    }
}

impl<T: FromJsValue> FromJsValue for Option<T> {
    type WasmAbi = JsValue; // TODO: better ABI?

    fn from_js_value(value: &JsValue) -> Result<Self> {
        if value.is_undefined() || value.is_null() {
            Ok(None)
        } else {
            Ok(Some(T::from_js_value(value)?))
        }
    }

    fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self> {
        Self::from_js_value(&abi)
    }
}

impl<T: FromJsValue> FromJsValue for Vec<T> {
    type WasmAbi = JsValue;

    fn from_js_value(value: &JsValue) -> Result<Self> {
        // TODO: Add user error?
        let length = Reflect::get(&value, &"length".into())?;
        let length = length.as_f64().unwrap() as u32;

        let mut result = Vec::with_capacity(length as usize);

        for index in 0..length {
            let item = Reflect::get_u32(value, index)?;
            result.push(T::from_js_value(&item)?);
        }

        Ok(result)
    }

    fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self> {
        Self::from_js_value(&abi)
    }
}

impl<T: FromJsValue> FromJsValue for (T,) {
    type WasmAbi = T::WasmAbi;

    fn from_js_value(value: &JsValue) -> Result<Self> {
        Ok((T::from_js_value(value)?,))
    }

    fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self> {
        Ok((T::from_wasm_abi(abi)?,))
    }
}

macro_rules! from_js_value_many {
    ($(($index: tt, $name: ident)),*) => {
        impl<$($name: FromJsValue),*> FromJsValue for ($($name, )*) {
            type WasmAbi = JsValue;

            fn from_js_value(results: &JsValue) -> Result<Self> {
                Ok(( $($name::from_js_value(&Reflect::get_u32(results, $index)?)?,)* ))
            }

            fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self> {
                Self::from_js_value(&abi)
            }
        }
    };
}

#[rustfmt::skip]
from_js_value_many!((0, T0), (1, T1));
#[rustfmt::skip]
from_js_value_many!((0, T0), (1, T1), (2, T2));
#[rustfmt::skip]
from_js_value_many!((0, T0), (1, T1), (2, T2), (3, T3));
#[rustfmt::skip]
from_js_value_many!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
#[rustfmt::skip]
from_js_value_many!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
#[rustfmt::skip]
from_js_value_many!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6));
#[rustfmt::skip]
from_js_value_many!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7));
#[rustfmt::skip]
from_js_value_many!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8));
#[rustfmt::skip]
from_js_value_many!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9));
#[rustfmt::skip]
from_js_value_many!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9), (10, T10));
#[rustfmt::skip]
from_js_value_many!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9), (10, T10), (11, T11));
