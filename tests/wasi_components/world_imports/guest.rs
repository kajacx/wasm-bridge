wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "wit-imports",
});

struct GuestImpl;

impl Guest for GuestImpl {
    fn add_three(num: i32) -> i32 {
        let num = add_one(num);
        let num = add_one(num);
        let num = add_one(num);
        num
    }

    fn push_strings(strings: Vec<String>, a: String, b: String) -> Vec<String> {
        let strings = push_string(&strings, &a);
        let strings = push_string(&strings, &b);
        strings
    }
}

export!(GuestImpl);
