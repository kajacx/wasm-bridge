// Single value
#[no_mangle]
pub fn add_three_i32(number: i32) -> i32 {
    unsafe { add_one_i32(number.wrapping_add(1)).wrapping_add(1) }
}

#[no_mangle]
pub fn add_three_i64(number: i64) -> i64 {
    unsafe { add_one_i64(number.wrapping_add(1)).wrapping_add(1) }
}

#[no_mangle]
pub fn add_three_f32(number: f32) -> f32 {
    unsafe { add_one_f32(number + 1.0) + 1.0 }
}

#[no_mangle]
pub fn add_three_f64(number: f64) -> f64 {
    unsafe { add_one_f64(number + 1.0) + 1.0 }
}

// Multiple params
#[no_mangle]
pub fn add_i32(a: i32, b: i32) -> i32 {
    unsafe { add_i32_import(a, b) }
}

// Multiple results
#[no_mangle]
pub fn add_sub_ten(num: i32) -> (i32, i32) {
    unsafe { add_sub_ten_import(num) }
}

// No arguments
#[no_mangle]
pub fn increment_twice() {
    unsafe {
        increment();
        increment();
    }
}

// // Panic in imported fn
// #[no_mangle]
// pub fn panics() {
//     unsafe {
//         panics_import();
//     }
// }

#[link(wasm_import_module = "imported_fns")]
extern "C" {
    fn add_one_i32(number: i32) -> i32;
    fn add_one_i64(number: i64) -> i64;
    fn add_one_f32(number: f32) -> f32;
    fn add_one_f64(number: f64) -> f64;

    fn add_i32_import(a: i32, b: i32) -> i32;
    #[allow(improper_ctypes)]
    fn add_sub_ten_import(num: i32) -> (i32, i32);

    fn increment();
    // fn panics_import();
}
