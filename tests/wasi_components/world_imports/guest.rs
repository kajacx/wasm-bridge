wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "wit-imports",
    exports: {
        world: GuestImpl
    }
});

struct GuestImpl;

impl Guest for GuestImpl {
    fn add_three(num: i32) -> i32 {
        let num = add_one(num);
        let num = add_one(num);
        let num = add_one(num);
        num
    }
}
