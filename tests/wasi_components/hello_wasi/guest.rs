wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "test-world",
});

struct GuestImpl;

impl TestWorld for GuestImpl {
    fn add_three(num: i32) -> i32 {
        num + 3
    }
}

export_test_world!(GuestImpl);
