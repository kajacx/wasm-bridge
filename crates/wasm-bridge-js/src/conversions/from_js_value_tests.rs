use super::*;

fn test_eq<T: FromJsValue + PartialEq + std::fmt::Debug>(js_str: &str, val: T) {
    let js_val = js_sys::eval(js_str).unwrap();
    let result = T::from_js_value(&js_val).unwrap();
    assert_eq!(result, val);
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn test_i8() {
    test_eq("5", 5i8);
    test_eq("-10", -10i8);
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn test_i16() {
    test_eq("5", 5i16);
    test_eq("-10", -10i16);
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn test_i32() {
    test_eq("5", 5i32);
    test_eq("-10", -10i32);
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn test_i64() {
    test_eq("5n", 5i64);
    test_eq("-10n", -10i64);
    test_eq("1000000000000000000n", 1_000_000_000_000_000_000i64);
    test_eq("-1000000000000000000n", -1_000_000_000_000_000_000i64);
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn test_u8() {
    test_eq("5", 5u8);
    test_eq("200", 200u8);
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn test_u16() {
    test_eq("5", 5u16);
    test_eq("60000", 60_000u16);
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn test_u32() {
    test_eq("5", 5u32);
    test_eq("4000000000", 4_000_000_000u32);
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn test_u64() {
    test_eq("5n", 5u64);
    test_eq("18000000000000000000n", 18_000_000_000_000_000_000u64);
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn test_f32() {
    test_eq("3.5", 3.5f32);
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn test_f64() {
    test_eq("3.14159265359", 3.14159265359f64)
}
