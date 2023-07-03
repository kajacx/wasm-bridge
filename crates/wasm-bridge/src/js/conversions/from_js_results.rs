use js_sys::Reflect;
use wasm_bindgen::JsValue;

use crate::*;

pub trait FromJsResults: Sized {
    // fn number_of_results() -> u32; TODO: check return length?
    fn from_js_results(results: &JsValue) -> Result<Self, Error>;
}

impl FromJsResults for () {
    fn from_js_results(_results: &JsValue) -> Result<Self, Error> {
        Ok(())
    }
}

macro_rules! from_js_results_single {
    ($ty: ty) => {
        impl FromJsResults for $ty {
            fn from_js_results(results: &JsValue) -> Result<Self, Error> {
                <Self as FromJsValue>::from_js_value(results)
            }
        }

        impl FromJsResults for ($ty,) {
            fn from_js_results(results: &JsValue) -> Result<Self, Error> {
                Ok((<$ty as FromJsValue>::from_js_value(results)?,))
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

macro_rules! from_js_results_many {
    ($(($index: tt, $name: ident)),*) => {
        impl<$($name: FromJsValue),*> FromJsResults for ($($name, )*) {
            fn from_js_results(results: &JsValue) -> Result<Self, Error> {
                Ok(( $($name::from_js_value(&Reflect::get_u32(results, $index)?)?,)* ))
            }
        }
    };
}

#[rustfmt::skip]
from_js_results_many!((0, T0), (1, T1));
#[rustfmt::skip]
from_js_results_many!((0, T0), (1, T1), (2, T2));
#[rustfmt::skip]
from_js_results_many!((0, T0), (1, T1), (2, T2), (3, T3));
#[rustfmt::skip]
from_js_results_many!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
#[rustfmt::skip]
from_js_results_many!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
#[rustfmt::skip]
from_js_results_many!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6));
#[rustfmt::skip]
from_js_results_many!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7));
#[rustfmt::skip]
from_js_results_many!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8));
#[rustfmt::skip]
from_js_results_many!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9));
#[rustfmt::skip]
from_js_results_many!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9), (10, T10));
#[rustfmt::skip]
from_js_results_many!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9), (10, T10), (11, T11));
