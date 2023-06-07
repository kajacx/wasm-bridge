use wasm_bindgen::JsValue;

use crate::*;

pub trait FromJsValue: Sized {
    fn from_js_value(value: &JsValue) -> Result<Self, Error>;
}

impl FromJsValue for i32 {
    fn from_js_value(value: &JsValue) -> Result<Self, Error> {
        match value.as_f64() {
            Some(number) => Ok(number as _),
            None => Err(Error::JsError(value.clone())), // TODO: better error, in other types too
        }
    }
}

impl FromJsValue for u32 {
    fn from_js_value(value: &JsValue) -> Result<Self, Error> {
        match value.as_f64() {
            // Conversion to i32 first needed to handle "negative" numbers
            Some(number) => Ok(number as i32 as _),
            None => Err(Error::JsError(value.clone())),
        }
    }
}

impl FromJsValue for f32 {
    fn from_js_value(value: &JsValue) -> Result<Self, Error> {
        match value.as_f64() {
            Some(number) => Ok(number as _),
            None => Err(Error::JsError(value.clone())),
        }
    }
}
