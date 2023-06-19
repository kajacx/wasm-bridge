#[no_mangle]
pub fn add_i32(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}
