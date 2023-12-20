wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "simple",
    exports: {
        world: MyGuest,
    }
});

struct MyGuest;

impl Guest for MyGuest {
    fn push_s32s(mut numbers: Vec<i32>, a: i32, b: i32) -> Vec<i32> {
        numbers.push(a);
        numbers.push(b);
        numbers
    }

    fn push_u32s(mut numbers: Vec<u32>, a: u32, b: u32) -> Vec<u32> {
        numbers.push(a);
        numbers.push(b);
        numbers
    }

    fn voider() {}

    fn pairs() -> (i32, i32) {
        (0, 1)
    }
}
