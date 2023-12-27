wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "tuples",
    exports: {
        world: MyGuest,
    }
});

struct MyGuest;

impl Guest for MyGuest {
    fn add_sub_one(num: i32) -> (i32, i32) {
        let (a, b) = add_sub_two(num);
        (a - 1, b + 1)
    }

    fn add_sub_twenty(num: i32) -> (i32, i32) {
        let (a, b) = add_sub_ten(num);
        (a + 10, b - 10)
    }
}
