#[no_mangle]
pub fn add_three_i32(number: i32) -> i32 {
    unsafe {
        let number = add_one_i32(number);
        let number = add_one_i32(number);
        let number = add_one_i32(number);
        number
    }
}

#[no_mangle]
pub fn add_three_i64(number: i64) -> i64 {
    unsafe {
        let number = add_one_i64(number);
        let number = add_one_i64(number);
        let number = add_one_i64(number);
        number
    }
}

#[no_mangle]
pub fn add_three_f32(number: f32) -> f32 {
    unsafe {
        let number = add_one_f32(number);
        let number = add_one_f32(number);
        let number = add_one_f32(number);
        number
    }
}

#[no_mangle]
pub fn add_three_f64(number: f64) -> f64 {
    unsafe {
        let number = add_one_f64(number);
        let number = add_one_f64(number);
        let number = add_one_f64(number);
        number
    }
}

#[link(wasm_import_module = "imported_fns")]
extern "C" {
    fn add_one_i32(number: i32) -> i32;
    fn add_one_i64(number: i64) -> i64;
    fn add_one_f32(number: f32) -> f32;
    fn add_one_f64(number: f64) -> f64;
}
