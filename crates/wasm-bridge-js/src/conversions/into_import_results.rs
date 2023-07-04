use wasm_bindgen::convert::ReturnWasmAbi;
use wasm_bindgen::JsValue;

use crate::*;

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

macro_rules! impl_into_import_results_single {
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

impl_into_import_results_single!(i32);
impl_into_import_results_single!(i64);
impl_into_import_results_single!(u32);
impl_into_import_results_single!(u64);
impl_into_import_results_single!(f32);
impl_into_import_results_single!(f64);

macro_rules! impl_into_import_results_many {
    ($wrapper: ident, $(($index: tt, $name: ident)),*) => {
        impl<$($name: Into<JsValue>),*> IntoImportResults for ($($name),*) {
            type Results = $wrapper<$($name),*>;

            fn into_import_results(self) -> Self::Results {
                $wrapper($(self.$index),*)
            }
        }
    };
}

#[rustfmt::skip]
impl_into_import_results_many!(TupleWrapper2,  (0, T0), (1, T1));
#[rustfmt::skip]
impl_into_import_results_many!(TupleWrapper3,  (0, T0), (1, T1), (2, T2));
#[rustfmt::skip]
impl_into_import_results_many!(TupleWrapper4,  (0, T0), (1, T1), (2, T2), (3, T3));
#[rustfmt::skip]
impl_into_import_results_many!(TupleWrapper5,  (0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
#[rustfmt::skip]
impl_into_import_results_many!(TupleWrapper6,  (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
#[rustfmt::skip]
impl_into_import_results_many!(TupleWrapper7,  (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6));
#[rustfmt::skip]
impl_into_import_results_many!(TupleWrapper8,  (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7));
#[rustfmt::skip]
impl_into_import_results_many!(TupleWrapper9,  (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8));
#[rustfmt::skip]
impl_into_import_results_many!(TupleWrapper10, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9));
#[rustfmt::skip]
impl_into_import_results_many!(TupleWrapper11, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9), (10, T10));
#[rustfmt::skip]
impl_into_import_results_many!(TupleWrapper12, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9), (10, T10), (11, T11));
