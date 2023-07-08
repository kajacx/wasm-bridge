use js_sys::{Array, Reflect};
use wasm_bindgen::JsValue;

pub trait IntoJsParams {
    fn number_of_args() -> u32;
    fn to_js_params(self) -> Array;
}

macro_rules! into_js_params_single {
    ($ty: ty) => {
        impl IntoJsParams for $ty {
            fn number_of_args() -> u32 {
                1
            }

            fn to_js_params(self) -> Array {
                Array::of1(&self.into())
            }
        }
    };
}

into_js_params_single!(i32);
into_js_params_single!(i64);
into_js_params_single!(u32);
into_js_params_single!(u64);
into_js_params_single!(f32);
into_js_params_single!(f64);

macro_rules! into_js_params_many {
    ($count: literal, $(($index: tt, $name: ident)),*) => {
        impl<$($name: Into<JsValue>),*> IntoJsParams for ($($name, )*) {
            fn number_of_args() -> u32 {
                $count
            }

            fn to_js_params(self) -> Array {
                // TODO: test is "ofN" is faster, and by how much
                let result = Array::new_with_length($count);
                $( Reflect::set_u32(&result, $index, &self.$index.into()).unwrap(); )*
                result
            }
        }
    };
}

#[rustfmt::skip]
into_js_params_many!( 0, );
#[rustfmt::skip]
into_js_params_many!( 1, (0, T0));
#[rustfmt::skip]
into_js_params_many!( 2, (0, T0), (1, T1));
#[rustfmt::skip]
into_js_params_many!( 3, (0, T0), (1, T1), (2, T2));
#[rustfmt::skip]
into_js_params_many!( 4, (0, T0), (1, T1), (2, T2), (3, T3));
#[rustfmt::skip]
into_js_params_many!( 5, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
#[rustfmt::skip]
into_js_params_many!( 6, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
#[rustfmt::skip]
into_js_params_many!( 7, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6));
#[rustfmt::skip]
into_js_params_many!( 8, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7));
#[rustfmt::skip]
into_js_params_many!( 9, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8));
#[rustfmt::skip]
into_js_params_many!(10, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9));
#[rustfmt::skip]
into_js_params_many!(11, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9), (10, T10));
#[rustfmt::skip]
into_js_params_many!(12, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9), (10, T10), (11, T11));
