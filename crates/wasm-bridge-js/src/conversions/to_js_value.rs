use js_sys::{
    Array, BigInt64Array, BigUint64Array, Float32Array, Float64Array, Int16Array, Int32Array,
    Int8Array, Object, Reflect, Uint16Array, Uint32Array, Uint8Array,
};
use wasm_bindgen::{convert::ReturnWasmAbi, JsValue};

pub trait ToJsValue: Sized {
    type ReturnAbi: ReturnWasmAbi;

    fn to_js_value(&self) -> JsValue;

    /// When this is returned from a closure
    fn into_return_abi(self) -> Self::ReturnAbi;

    /// Number of function arguments when this type is used as a function input type
    fn number_of_args() -> u32 {
        1
    }

    /// Convert to function arguments when calling a function with this value
    fn to_function_args(&self) -> Array {
        Array::of1(&self.to_js_value())
    }

    /// When converting Vec<Self> to JsValue, create array or Int32Array for example
    fn create_array_of_size(size: u32) -> JsValue {
        Array::new_with_length(size).into()
    }
}

impl ToJsValue for () {
    type ReturnAbi = Self;

    fn to_js_value(&self) -> JsValue {
        JsValue::undefined()
    }

    fn into_return_abi(self) -> Self::ReturnAbi {}

    fn number_of_args() -> u32 {
        0
    }

    fn to_function_args(&self) -> Array {
        Array::new()
    }
}

macro_rules! to_js_value_single {
    ($ty: ty, $array: ty) => {
        impl ToJsValue for $ty {
            type ReturnAbi = Self;

            fn to_js_value(&self) -> JsValue {
                (*self).into()
            }

            fn into_return_abi(self) -> Self::ReturnAbi {
                self
            }

            fn create_array_of_size(size: u32) -> JsValue {
                <$array>::new_with_length(size).into()
            }
        }
    };
}

to_js_value_single!(bool, Array);

to_js_value_single!(i8, Int8Array);
to_js_value_single!(i16, Int16Array);
to_js_value_single!(i32, Int32Array);
to_js_value_single!(i64, BigInt64Array);

to_js_value_single!(u8, Uint8Array);
to_js_value_single!(u16, Uint16Array);
to_js_value_single!(u32, Uint32Array);
to_js_value_single!(u64, BigUint64Array);

to_js_value_single!(f32, Float32Array);
to_js_value_single!(f64, Float64Array);

impl ToJsValue for char {
    type ReturnAbi = Self;

    fn to_js_value(&self) -> JsValue {
        // TODO: not really great copy
        self.to_string().into()
    }

    fn into_return_abi(self) -> Self::ReturnAbi {
        self
    }
}

impl<'a> ToJsValue for &'a str {
    type ReturnAbi = Self;

    fn to_js_value(&self) -> JsValue {
        (*self).into()
    }

    fn into_return_abi(self) -> Self::ReturnAbi {
        self
    }
}

impl ToJsValue for String {
    type ReturnAbi = Self;

    fn to_js_value(&self) -> JsValue {
        self.into()
    }

    fn into_return_abi(self) -> Self::ReturnAbi {
        self.clone() // TODO: unnecessary copy ... ?
    }
}

// TODO: inspect OptionIntoWasmAbi and see if it's better
impl<T: ToJsValue> ToJsValue for Option<T> {
    type ReturnAbi = JsValue;

    fn to_js_value(&self) -> JsValue {
        match self {
            Self::Some(value) => value.to_js_value(),
            None => JsValue::undefined(),
        }
    }

    fn into_return_abi(self) -> Self::ReturnAbi {
        self.to_js_value()
    }
}

impl<T: ToJsValue, E: ToJsValue> ToJsValue for Result<T, E> {
    type ReturnAbi = Result<JsValue, JsValue>;

    fn to_js_value(&self) -> JsValue {
        let result: JsValue = Object::new().into();
        let (tag, val) = match self {
            Ok(value) => ("ok", value.to_js_value()),
            Err(err) => ("err", err.to_js_value()),
        };
        Reflect::set(&result, &"tag".into(), &tag.into()).unwrap();
        Reflect::set(&result, &"val".into(), &val).unwrap();
        result
    }

    fn into_return_abi(self) -> Self::ReturnAbi {
        self.as_ref().map(T::to_js_value).map_err(E::to_js_value)
    }
}

impl<'a, T: ToJsValue> ToJsValue for &'a T {
    type ReturnAbi = T::ReturnAbi;

    fn to_js_value(&self) -> JsValue {
        T::to_js_value(self)
    }

    fn into_return_abi(self) -> Self::ReturnAbi {
        unimplemented!("References should never be returned")
    }
}

// TODO: unify references...
impl<'a, T: ToJsValue> ToJsValue for &'a [T] {
    type ReturnAbi = JsValue;

    fn to_js_value(&self) -> JsValue {
        let array = T::create_array_of_size(self.len() as _);
        self.iter().enumerate().for_each(|(index, item)| {
            // TODO: set_index is probably faster to Int32Array and "friends"
            Reflect::set_u32(&array, index as _, &item.to_js_value()).expect("array is array");
        });
        array
    }

    fn into_return_abi(self) -> Self::ReturnAbi {
        self.to_js_value()
    }
}

impl<T: ToJsValue> ToJsValue for Vec<T> {
    type ReturnAbi = JsValue;

    fn to_js_value(&self) -> JsValue {
        let as_slice: &[T] = self;
        as_slice.to_js_value()
    }

    fn into_return_abi(self) -> Self::ReturnAbi {
        self.to_js_value()
    }
}

impl<T: ToJsValue> ToJsValue for (T,) {
    type ReturnAbi = T::ReturnAbi;

    fn to_js_value(&self) -> JsValue {
        self.0.to_js_value()
    }

    fn into_return_abi(self) -> Self::ReturnAbi {
        self.0.into_return_abi()
    }

    fn number_of_args() -> u32 {
        T::number_of_args()
    }

    fn to_function_args(&self) -> Array {
        self.0.to_function_args()
    }
}

macro_rules! to_js_value_many {
    ($count: literal, $(($index: tt, $name: ident)),*) => {
        impl<$($name: ToJsValue),*> ToJsValue for ($($name, )*) {
            type ReturnAbi = JsValue;

            fn to_js_value(&self) -> JsValue {
                self.to_function_args().into()
            }

            fn into_return_abi(self) -> Self::ReturnAbi {
                self.to_js_value()
            }

            fn number_of_args() -> u32 {
                $count
            }

            fn to_function_args(&self) -> Array {
                // TODO: test is "ofN" is faster, and by how much
                let result = Array::new_with_length($count);
                $( Reflect::set_u32(&result, $index, &self.$index.to_js_value()).expect("result is array"); )*
                result
            }
        }
    };
}

#[rustfmt::skip]
to_js_value_many!( 2, (0, T0), (1, T1));
#[rustfmt::skip]
to_js_value_many!( 3, (0, T0), (1, T1), (2, T2));
#[rustfmt::skip]
to_js_value_many!( 4, (0, T0), (1, T1), (2, T2), (3, T3));
#[rustfmt::skip]
to_js_value_many!( 5, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
#[rustfmt::skip]
to_js_value_many!( 6, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
#[rustfmt::skip]
to_js_value_many!( 7, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6));
#[rustfmt::skip]
to_js_value_many!( 8, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7));
#[rustfmt::skip]
to_js_value_many!( 9, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8));
#[rustfmt::skip]
to_js_value_many!(10, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9));
#[rustfmt::skip]
to_js_value_many!(11, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9), (10, T10));
#[rustfmt::skip]
to_js_value_many!(12, (0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9), (10, T10), (11, T11));
