use wasm_bindgen::JsValue;

use crate::*;

pub trait FromJsValue: Sized {
    fn from_js_value(value: &JsValue) -> Result<Self, Error>;
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
