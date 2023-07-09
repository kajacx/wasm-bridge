use js_sys::{Array, Reflect};
use wasm_bindgen::JsValue;

pub trait IntoImportResults {
    fn into_import_results(self) -> JsValue;
}

impl IntoImportResults for () {
    fn into_import_results(self) -> JsValue {
        JsValue::undefined()
    }
}

macro_rules! into_import_results_single {
    ($ty:ty) => {
        impl IntoImportResults for $ty {
            fn into_import_results(self) -> JsValue {
                self.into()
            }
        }
    };
}

impl<T: IntoImportResults> IntoImportResults for (T,) {
    fn into_import_results(self) -> JsValue {
        self.0.into_import_results()
    }
}

into_import_results_single!(i32);
into_import_results_single!(i64);
into_import_results_single!(u32);
into_import_results_single!(u64);
into_import_results_single!(f32);
into_import_results_single!(f64);
into_import_results_single!(String);

macro_rules! into_import_results_many {
    ($count: literal, $(($index: tt, $name: ident)),*) => {
        impl<$($name: IntoImportResults),*> IntoImportResults for ($($name),*) {
            fn into_import_results(self) -> JsValue {
                // TODO: Array::ofN might be faster
                let result: JsValue = Array::new_with_length($count).into();
                $(Reflect::set_u32(&result, $index, &self.$index.into_import_results()).expect("result is array");)*
                result
            }
        }
    };
}

#[rustfmt::skip]
into_import_results_many!(2,  (0, T0), (1, T1));
#[rustfmt::skip]
into_import_results_many!(3,  (0, T0), (1, T1), (2, T2));
#[rustfmt::skip]
into_import_results_many!(4,  (0, T0), (1, T1), (2, T2), (3, T3));
#[rustfmt::skip]
into_import_results_many!(5,  (0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
#[rustfmt::skip]
into_import_results_many!(6,  (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
#[rustfmt::skip]
into_import_results_many!(7,  (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6));
#[rustfmt::skip]
into_import_results_many!(8,  (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7));
#[rustfmt::skip]
into_import_results_many!(9,  (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8));
#[rustfmt::skip]
into_import_results_many!(10, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9));
#[rustfmt::skip]
into_import_results_many!(11, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9), (10, T10));
#[rustfmt::skip]
into_import_results_many!(12, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9), (10, T10), (11, T11));
