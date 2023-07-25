use anyhow::Context;
use js_sys::Reflect;
use wasm_bindgen::{convert::FromWasmAbi, JsValue};

use crate::{helpers::map_js_error, *};

pub trait FromJsValue: Sized {
    type WasmAbi: FromWasmAbi;

    fn from_js_value(value: &JsValue) -> Result<Self>;

    // When type is the (direct) result of an exported function
    fn from_fn_result(result: &Result<JsValue, JsValue>) -> Result<Self> {
        Self::from_js_value(
            result
                .as_ref()
                .map_err(map_js_error("Exported function threw an exception"))?,
        )
    }

    // When type is an argument of an imported function
    fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self>;
}

impl FromJsValue for () {
    type WasmAbi = JsValue;

    fn from_js_value(value: &JsValue) -> Result<Self> {
        if value.is_undefined() || value.is_null() {
            Ok(())
        } else {
            Err(map_js_error("Expected null or undefined")(value))
        }
    }

    fn from_wasm_abi(_abi: Self::WasmAbi) -> Result<Self> {
        Ok(())
    }
}

impl FromJsValue for bool {
    type WasmAbi = Self;

    fn from_js_value(value: &JsValue) -> Result<Self> {
        match value.as_bool() {
            Some(value) => Ok(value),
            None => Err(map_js_error("Expected a boolean value")(value)),
        }
    }

    fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self> {
        Ok(abi)
    }
}

macro_rules! from_js_value_signed {
    ($name: ty) => {
        impl FromJsValue for $name {
            type WasmAbi = Self;

            fn from_js_value(value: &JsValue) -> Result<Self> {
                match value.as_f64() {
                    Some(number) => Ok(number as _),
                    None => Err(map_js_error("Expected a number")(value)),
                }
            }

            fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self> {
                Ok(abi)
            }
        }
    };
}

from_js_value_signed!(i8);
from_js_value_signed!(i16);
from_js_value_signed!(i32);

impl FromJsValue for i64 {
    type WasmAbi = Self;

    fn from_js_value(value: &JsValue) -> Result<Self> {
        value
            .clone()
            .try_into()
            .map_err(map_js_error("Expected a bigint"))
    }

    fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self> {
        Ok(abi)
    }
}
macro_rules! from_js_value_unsigned {
    ($name: ty, $signed: ty) => {
        impl FromJsValue for $name {
            type WasmAbi = Self;

            fn from_js_value(value: &JsValue) -> Result<Self> {
                // TODO: Add a check that the value didn't overflow?
                match value.as_f64() {
                    // Value might be bigger than $name::MAX / 2 or smaller than 0
                    Some(number) if number < 0.0 => Ok(number as $signed as _),
                    Some(number) => Ok(number as _),
                    None => Err(map_js_error("Expected a number")(value)),
                }
            }

            fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self> {
                Ok(abi)
            }
        }
    };
}

from_js_value_unsigned!(u8, i8);
from_js_value_unsigned!(u16, i16);
from_js_value_unsigned!(u32, i32);

impl FromJsValue for u64 {
    type WasmAbi = Self;

    fn from_js_value(value: &JsValue) -> Result<Self> {
        // Value is BigInt, but it might be positive over u64::MAX / 2, or negative
        u64::try_from(value.clone())
            .or_else(|_| i64::try_from(value.clone()).map(|value| value as u64))
            .map_err(map_js_error("Expected a bigint"))
    }

    fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self> {
        Ok(abi)
    }
}

// TODO: not the best name, but it works
from_js_value_signed!(f32);
from_js_value_signed!(f64);

impl FromJsValue for char {
    type WasmAbi = Self;

    fn from_js_value(value: &JsValue) -> Result<Self> {
        match value.as_string() {
            Some(text) if !text.is_empty() => Ok(text.chars().next().unwrap()),
            _ => Err(map_js_error("Expected a single-character string")(value)),
        }
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
            None => Err(map_js_error("Expected a string")(value)),
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

impl<T: FromJsValue, E: FromJsValue> FromJsValue for Result<T, E> {
    type WasmAbi = JsValue;

    fn from_js_value(value: &JsValue) -> Result<Self> {
        // TODO: better error handling
        let tag = Reflect::get(value, &"tag".into())
            .map_err(map_js_error("Get tag from result"))?
            .as_string()
            .context("Result tag should be string")?;

        let val =
            Reflect::get(value, &"val".into()).map_err(map_js_error("Get val from result"))?;

        if tag == "ok" {
            Ok(Ok(T::from_js_value(&val)?))
        } else if tag == "err" {
            Ok(Err(E::from_js_value(&val)?))
        } else {
            Err(map_js_error("Unknown result tag")(value))
        }
    }

    fn from_fn_result(result: &Result<JsValue, JsValue>) -> Result<Self> {
        Ok(match result {
            Ok(val) => Ok(T::from_js_value(val)?),
            Err(err) => {
                let payload = Reflect::get(err, &"payload".into())
                    .map_err(map_js_error("Get result error payload"))?;
                Err(E::from_js_value(&payload)?)
            }
        })
    }

    fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self> {
        Self::from_js_value(&abi)
    }
}

impl<T: FromJsValue> FromJsValue for Vec<T> {
    type WasmAbi = JsValue;

    fn from_js_value(value: &JsValue) -> Result<Self> {
        let length = Reflect::get(value, &"length".into())
            .map_err(map_js_error("Get length of array"))?
            .as_f64()
            .context("Array length should be a number")? as u32;

        let mut result = Vec::with_capacity(length as usize);

        for index in 0..length {
            let item = Reflect::get_u32(value, index)
                .map_err(map_js_error("Get array value at an index"))?;
            result.push(T::from_js_value(&item)?);
        }

        Ok(result)
    }

    fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self> {
        Self::from_js_value(&abi)
    }
}

impl FromJsValue for JsValue {
    type WasmAbi = Self;

    fn from_js_value(value: &JsValue) -> Result<Self> {
        Ok(value.clone())
    }

    fn from_wasm_abi(abi: Self::WasmAbi) -> Result<Self> {
        Ok(abi)
    }
}

impl FromJsValue for Val {
    type WasmAbi = JsValue;

    fn from_js_value(value: &JsValue) -> Result<Self> {
        if let Some(number) = value.as_f64() {
            // TODO: BIG problem ... this could be I32, F32 or I64, and we don't really know which one ...
            Ok(Self::F64(number))
        } else if value.is_bigint() {
            // TODO: u64 is used, because it's more "robust" ... make i64 robust as well instead?
            Ok(Val::I64(u64::from_js_value(value)? as _))
        } else {
            Err(map_js_error("Unsupported 'Val' value")(value))
        }
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

    fn from_fn_result(result: &Result<JsValue, JsValue>) -> Result<Self> {
        Ok((T::from_fn_result(result)?,))
    }
}

macro_rules! from_js_value_many {
    ($(($index: tt, $name: ident)),*) => {
        impl<$($name: FromJsValue),*> FromJsValue for ($($name, )*) {
            type WasmAbi = JsValue;

            fn from_js_value(results: &JsValue) -> Result<Self> {
                Ok(( $($name::from_js_value(&Reflect::get_u32(results, $index).map_err(map_js_error("Get tuple value"))?)?,)* ))
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
