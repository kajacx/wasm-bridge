use super::*;
use wasm_bindgen::JsValue;

macro_rules! test_simple {
    ($ty: ty, $name: ident) => {
        #[wasm_bindgen_test::wasm_bindgen_test]
        fn $name() {
            for num in [
                <$ty>::MIN + 2,
                2,
                <$ty>::MAX / 2 - 2,
                <$ty>::MAX / 2 + 2,
                <$ty>::MAX - 2,
            ] {
                let number: JsValue = (num as f64).into();
                assert_eq!(<$ty>::from_js_value(&number).unwrap(), num);
            }
        }
    };
}

test_simple!(i8, test_i8);
test_simple!(i16, test_i16);
test_simple!(i32, test_i32);

test_simple!(u8, test_u8);
test_simple!(u16, test_u16);
test_simple!(u32, test_u32);

macro_rules! test_unsigned {
    ($ty: ty, $name: ident) => {
        #[wasm_bindgen_test::wasm_bindgen_test]
        fn $name() {
            let num = <$ty>::MAX - 2;
            let number: JsValue = (num as f64 - <$ty>::MAX as f64 - 1f64).into();
            assert_eq!(<$ty>::from_js_value(&number).unwrap(), num);
        }
    };
}

test_unsigned!(u8, unsigned_u8);
test_unsigned!(u16, unsigned_u16);
test_unsigned!(u32, unsigned_u32);

#[wasm_bindgen_test::wasm_bindgen_test]
fn test_i64() {
    for num in [
        i64::MIN + 2,
        2,
        i64::MAX / 2 - 2,
        i64::MAX / 2 + 2,
        i64::MAX - 2,
    ] {
        let number: JsValue = num.into();
        assert_eq!(i64::from_js_value(&number).unwrap(), num);
    }
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn test_u64() {
    for num in [
        i64::MIN + 2,
        2,
        i64::MAX / 2 - 2,
        i64::MAX / 2 + 2,
        i64::MAX - 2,
    ] {
        let number: JsValue = num.into();
        assert_eq!(u64::from_js_value(&number).unwrap(), num as u64);
    }

    for num in [2, u64::MAX / 2 - 2, u64::MAX / 2 + 2, u64::MAX - 2] {
        let number: JsValue = num.into();
        assert_eq!(u64::from_js_value(&number).unwrap(), num);
    }
}
