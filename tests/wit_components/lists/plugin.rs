wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "test-world",
});

struct Plugin;

impl TestWorld for Plugin {
    fn push_s32s(numbers: Vec<i32>, a: i32, b: i32) -> Vec<i32> {
        let numbers = push_s32(&numbers, a);
        let numbers = push_s32(&numbers, b);
        numbers
    }

    fn push_s64s(numbers: Vec<i64>, a: i64, b: i64) -> Vec<i64> {
        let numbers = push_s64(&numbers, a);
        let numbers = push_s64(&numbers, b);
        numbers
    }

    fn push_u32s(numbers: Vec<u32>, a: u32, b: u32) -> Vec<u32> {
        let numbers = push_u32(&numbers, a);
        let numbers = push_u32(&numbers, b);
        numbers
    }

    fn push_u64s(numbers: Vec<u64>, a: u64, b: u64) -> Vec<u64> {
        let numbers = push_u64(&numbers, a);
        let numbers = push_u64(&numbers, b);
        numbers
    }

    fn push_float32s(numbers: Vec<f32>, a: f32, b: f32) -> Vec<f32> {
        let numbers = push_float32(&numbers, a);
        let numbers = push_float32(&numbers, b);
        numbers
    }

    fn push_float64s(numbers: Vec<f64>, a: f64, b: f64) -> Vec<f64> {
        let numbers = push_float64(&numbers, a);
        let numbers = push_float64(&numbers, b);
        numbers
    }

    fn push_strings(strings: Vec<String>, a: String, b: String) -> Vec<String> {
        let strings = strings.iter().map(String::as_str).collect::<Vec<_>>();
        let strings = push_string(&strings, &a);

        let strings = strings.iter().map(String::as_str).collect::<Vec<_>>();
        let strings = push_string(&strings, &b);

        strings
    }
}

export_test_world!(Plugin);
