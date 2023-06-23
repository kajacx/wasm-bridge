use wasm_bindgen::convert::ReturnWasmAbi;
use wasm_bindgen::JsValue;

use crate::TupleWrapper2;

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
    ($count: ident, $(($index: tt, $name: ident)),*) => {
        impl<$($name: Into<JsValue>),*> IntoImportResults for ($($name),*) {
            type Results = $count<$($name),*>;

            fn into_import_results(self) -> Self::Results {
                $count($(self.$index),*)
            }
        }
    };
}

impl_into_import_results_many!(TupleWrapper2, (0, T0), (1, T1));
