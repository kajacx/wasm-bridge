// Single value
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

// Multiple params
#[no_mangle]
pub fn add_i32(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}

// Multiple results
#[no_mangle]
pub fn add_sub_ten_i32(number: i32) -> (i32, i32) {
    (number.wrapping_add(10), number.wrapping_sub(10))
}

// Panic test
#[no_mangle]
pub fn panics() {
    panic!("Panic in guest code");
}
