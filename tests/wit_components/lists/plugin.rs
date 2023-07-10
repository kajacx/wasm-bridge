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
}

export_test_world!(Plugin);
