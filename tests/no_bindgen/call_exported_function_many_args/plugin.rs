#[no_mangle]
pub fn add_i32(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}

#[no_mangle]
pub fn add_sub_ten_i32(number: i32) -> (i32, i32) {
    (number.wrapping_add(10), number.wrapping_sub(10))
}

#[no_mangle]
pub fn add_sub_ten_i64(number: i64) -> (i64, i64) {
    (number.wrapping_add(10), number.wrapping_sub(10))
}

#[no_mangle]
pub fn add_all(a: i32, b: i64, c: u32, d: u64, e: f32, g: f64) -> f64 {
    a as f64 + b as f64 + c as f64 + d as f64 + e as f64 + g
}

// TODO: Rust cannot compile code returning more than 2 values?
// #[no_mangle]
// pub fn add_ten_i32s(a: i32, b: i32, c: i32) -> (i32, i32, i32, i32) {
//     (
//         a.wrapping_add(10),
//         b.wrapping_add(10),
//         c.wrapping_add(10),
//         d.wrapping_add(10),
//     )
// }
