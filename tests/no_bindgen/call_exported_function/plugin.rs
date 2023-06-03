#[no_mangle]
pub fn add_five_i32(number: i32) -> i32 {
    number.wrapping_add(5)
}
