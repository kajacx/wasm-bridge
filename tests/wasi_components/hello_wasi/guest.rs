wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "test-world",
});

struct GuestImpl;

impl TestWorld for GuestImpl {
    fn add_three(num: i32) -> i32 {
        let num = add_one(num);
        let num = add_one(num);
        let num = add_one(num);
        num
    }
}

export_test_world!(GuestImpl);
