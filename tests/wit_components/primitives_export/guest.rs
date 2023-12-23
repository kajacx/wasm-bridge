wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "primitives",
    exports: {
        world: MyGuest,
    }
});

struct MyGuest;

impl Guest for MyGuest {
    fn negate_times(mut value: bool, times: u32) -> bool {
        for _ in 0..times {
            value = !value;
        }
        value
    }

    fn add_three_s8(num: i8) -> i8 {
        num + 3
    }

    fn add_three_s16(num: i16) -> i16 {
        num + 3
    }

    fn add_three_s32(num: i32) -> i32 {
        num + 3
    }

    fn add_three_s64(num: i64) -> i64 {
        num + 3
    }

    fn add_three_u8(num: u8) -> u8 {
        num + 3
    }

    fn add_three_u16(num: u16) -> u16 {
        num + 3
    }

    fn add_three_u32(num: u32) -> u32 {
        num + 3
    }

    fn add_three_u64(num: u64) -> u64 {
        num + 3
    }

    fn add_three_float32(num: f32) -> f32 {
        num + 3.0
    }

    fn add_three_float64(num: f64) -> f64 {
        num + 3.0
    }

    fn to_upper(ch: char) -> char {
        ch.to_uppercase().next().unwrap()
    }

    fn add_abc(text: String) -> String {
        format!("{text}abc")
    }
}
