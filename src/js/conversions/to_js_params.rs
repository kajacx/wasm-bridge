use js_sys::Array;
use wasm_bindgen::JsValue;

pub trait ToJsParams: Copy {
    fn to_js_params(self) -> Array;
}

impl<T: Into<JsValue> + Copy, U: Into<JsValue> + Copy> ToJsParams for (T, U) {
    fn to_js_params(self) -> Array {
        Array::of2(&self.0.into(), &self.1.into())
    }
}

macro_rules! to_js_params_single {
    ($ty: ty) => {
        impl ToJsParams for $ty {
            fn to_js_params(self) -> Array {
                Array::of1(&self.into())
            }
        }
    };
}

to_js_params_single!(i32);
to_js_params_single!(i64);
to_js_params_single!(u32);
to_js_params_single!(u64);
to_js_params_single!(f32);
to_js_params_single!(f64);
