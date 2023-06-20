use js_sys::Reflect;
use wasm_bindgen::JsValue;

use crate::*;

pub trait FromJsResults: Sized {
    fn from_js_results(results: &JsValue) -> Result<Self, Error>;
}

impl<T: FromJsValue, U: FromJsValue> FromJsResults for (T, U) {
    fn from_js_results(results: &JsValue) -> Result<Self, Error> {
        let first = Reflect::get_u32(results, 0)?;
        let first = T::from_js_value(&first)?;

        let second = Reflect::get_u32(results, 1)?;
        let second = U::from_js_value(&second)?;

        Ok((first, second))
    }
}

macro_rules! from_js_results_single {
    ($ty: ty) => {
        impl FromJsResults for $ty {
            fn from_js_results(results: &JsValue) -> Result<Self, Error> {
                <Self as FromJsValue>::from_js_value(results)
            }
        }
    };
}

from_js_results_single!(i32);
from_js_results_single!(i64);
from_js_results_single!(u32);
from_js_results_single!(u64);
from_js_results_single!(f32);
from_js_results_single!(f64);
