use js_sys::{Array, Reflect};
use wasm_bindgen::convert::ReturnWasmAbi;
use wasm_bindgen::JsValue;

pub trait IntoImportResults {
    type Results: ReturnWasmAbi;

    fn into_import_results(self) -> Self::Results;
}

impl IntoImportResults for () {
    type Results = ();

    fn into_import_results(self) -> Self::Results {
        self
    }
}

macro_rules! into_import_results_single {
    ($ty:ty) => {
        impl IntoImportResults for $ty {
            type Results = Self;

            fn into_import_results(self) -> Self::Results {
                self
            }
        }

        impl IntoImportResults for ($ty,) {
            type Results = $ty;

            fn into_import_results(self) -> Self::Results {
                self.0
            }
        }
    };
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
        impl<$($name: Into<JsValue>),*> IntoImportResults for ($($name),*) {
            type Results = JsValue;

            fn into_import_results(self) -> Self::Results {
                // TODO: Array::ofN might be faster
                let result: JsValue = Array::new_with_length($count).into();
                $(Reflect::set_u32(&result, $index, &self.$index.into()).unwrap();)*
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
