#[no_mangle]
pub fn add_three_i32(number: i32) -> i32 {
    unsafe { add_one_i32(number.wrapping_add(1)).wrapping_add(1) }
}

#[link(wasm_import_module = "imported_fns")]
extern "C" {
    fn add_one_i32(number: i32) -> i32;
}
