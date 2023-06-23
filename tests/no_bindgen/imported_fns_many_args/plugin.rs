#[no_mangle]
pub fn add_i32(a: i32, b: i32) -> i32 {
    unsafe { add_i32_import(a, b) }
}

#[no_mangle]
pub fn add_sub_ten(num: i32) -> (i32, i32) {
    unsafe { add_sub_ten_import(num) }
}

#[link(wasm_import_module = "imported_fns")]
extern "C" {
    fn add_i32_import(a: i32, b: i32) -> i32;
    fn add_sub_ten_import(num: i32) -> (i32, i32);
}
