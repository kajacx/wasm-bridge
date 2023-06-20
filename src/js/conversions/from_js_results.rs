use wasm_bindgen::JsValue;

use crate::*;

pub trait FromJsResults: Sized {
    fn from_js_results(results: &JsValue) -> Result<Self, Error>;
}

// impl<T: Into<JsValue> + Copy, U: Into<JsValue> + Copy> ToJsParams for (T, U) {
//     fn to_js_params(self) -> Array {
//         Array::of2(&self.0.into(), &self.1.into())
//     }
// }

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
