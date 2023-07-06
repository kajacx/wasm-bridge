wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "primitives",
});

struct Plugin;

impl Primitives for Plugin {
    fn add_three_s32(num: i32) -> i32 {
        num + 3
    }

    fn add_three_s64(num: i64) -> i64 {
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

    fn add_abc(text: String) -> String {
        format!("{text}abc")
    }
}

export_primitives!(Plugin);
