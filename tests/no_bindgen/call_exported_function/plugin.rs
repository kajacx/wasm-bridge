#[no_mangle]
pub fn add_five_i32(number: i32) -> i32 {
    number.wrapping_add(5)
}

#[no_mangle]
pub fn add_five_i64(number: i64) -> i64 {
    number.wrapping_add(5)
}

#[no_mangle]
pub fn add_five_f32(number: f32) -> f32 {
    number + 5.0
}

#[no_mangle]
pub fn add_five_f64(number: f64) -> f64 {
    number + 5.0
}
