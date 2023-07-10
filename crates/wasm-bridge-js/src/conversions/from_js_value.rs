use js_sys::Reflect;
use wasm_bindgen::JsValue;

use crate::*;

pub trait FromJsValue: Sized {
    fn from_js_value(value: &JsValue) -> Result<Self, Error>;
}

impl FromJsValue for () {
    fn from_js_value(value: &JsValue) -> Result<Self, Error> {
        if value.is_undefined() || value.is_null() {
            Ok(())
        } else {
            Err(value.into())
        }
    }
}

impl FromJsValue for i32 {
    fn from_js_value(value: &JsValue) -> Result<Self, Error> {
        match value.as_f64() {
            Some(number) => Ok(number as _),
            None => Err(value.into()), // TODO: better error, in other types too
        }
    }
}

impl FromJsValue for i64 {
    fn from_js_value(value: &JsValue) -> Result<Self, Error> {
        Ok(value.clone().try_into()?)
    }
}

impl FromJsValue for u32 {
    fn from_js_value(value: &JsValue) -> Result<Self, Error> {
        match value.as_f64() {
            // Conversion to i32 first needed to handle "negative" numbers
            Some(number) => Ok(number as i32 as _),
            None => Err(value.into()),
        }
    }
}

impl FromJsValue for u64 {
    fn from_js_value(value: &JsValue) -> Result<Self, Error> {
        // Conversion to u32 first needed to handle "negative" numbers
        Ok(i64::try_from(value.clone())? as _)
    }
}

impl FromJsValue for f32 {
    fn from_js_value(value: &JsValue) -> Result<Self, Error> {
        match value.as_f64() {
            Some(number) => Ok(number as _),
            None => Err(value.into()),
        }
    }
}

impl FromJsValue for f64 {
    fn from_js_value(value: &JsValue) -> Result<Self, Error> {
        Ok(value.try_into()?)
    }
}

impl FromJsValue for String {
    fn from_js_value(value: &JsValue) -> Result<Self, crate::Error> {
        match value.as_string() {
            Some(value) => Ok(value),
            None => Err(value.into()), // TODO: better error
        }
    }
}

impl<T: FromJsValue> FromJsValue for Option<T> {
    fn from_js_value(value: &JsValue) -> Result<Self, Error> {
        if value.is_undefined() || value.is_null() {
            Ok(None)
        } else {
            Ok(Some(T::from_js_value(value)?))
        }
    }
}

impl<T: FromJsValue> FromJsValue for Vec<T> {
    fn from_js_value(value: &JsValue) -> Result<Self, Error> {
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
}

impl<T: FromJsValue> FromJsValue for (T,) {
    fn from_js_value(value: &JsValue) -> Result<Self, Error> {
        Ok((T::from_js_value(value)?,))
    }
}

macro_rules! from_js_value_many {
    ($(($index: tt, $name: ident)),*) => {
        impl<$($name: FromJsValue),*> FromJsValue for ($($name, )*) {
            fn from_js_value(results: &JsValue) -> Result<Self, Error> {
                Ok(( $($name::from_js_value(&Reflect::get_u32(results, $index)?)?,)* ))
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
