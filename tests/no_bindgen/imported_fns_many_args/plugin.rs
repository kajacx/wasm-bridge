#[no_mangle]
pub fn add_i32(a: i32, b: i32) -> i32 {
    unsafe { add_i32_import(a, b) }
}

#[link(wasm_import_module = "imported_fns")]
extern "C" {
    fn add_i32_import(a: i32, b: i32) -> i32;
}
