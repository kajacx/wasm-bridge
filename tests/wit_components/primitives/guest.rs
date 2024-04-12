wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "primitives",
});

struct GuestImpl;

impl Guest for GuestImpl {
    fn negate_times(mut value: bool, times: u32) -> bool {
        for _ in 0..times {
            value = negate(value);
        }
        value
    }

    fn add_three_s8(num: i8) -> i8 {
        add_one_s8(num + 1) + 1
    }

    fn add_three_s16(num: i16) -> i16 {
        add_one_s16(num + 1) + 1
    }

    fn add_three_s32(num: i32) -> i32 {
        add_one_s32(num + 1) + 1
    }

    fn add_three_s64(num: i64) -> i64 {
        add_one_s64(num + 1) + 1
    }

    fn add_three_u8(num: u8) -> u8 {
        add_one_u8(num + 1) + 1
    }

    fn add_three_u16(num: u16) -> u16 {
        add_one_u16(num + 1) + 1
    }

    fn add_three_u32(num: u32) -> u32 {
        add_one_u32(num + 1) + 1
    }

    fn add_three_u64(num: u64) -> u64 {
        add_one_u64(num + 1) + 1
    }

    fn add_three_float32(num: f32) -> f32 {
        add_one_float32(num + 1.0) + 1.0
    }

    fn add_three_float64(num: f64) -> f64 {
        add_one_float64(num + 1.0) + 1.0
    }

    fn to_upper(ch: char) -> char {
        to_upper_import(ch)
    }

    fn add_abc(text: String) -> String {
        let text = format!("{text}a");
        let text = add_b(&text);
        let text = format!("{text}c");
        text
    }
}

export!(GuestImpl);
