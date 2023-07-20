wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "test-world",
});

struct GuestImpl;

impl TestWorld for GuestImpl {
    fn promote_person(employee: Person, raise: u32) -> Person {
        set_salary(&employee, employee.salary + raise)
    }

    fn increment_twice() {
        increment();
        increment();
    }

    fn add_all_and_one(a: i32, b: i64, c: u32, d: u64, e: f32, f: f64, g: String) -> f64 {
        add_all(a, b, c, d, e, f, &g) + 1.0
    }

    fn add_sub_one(num: i32) -> (i32, i32) {
        let (a, b) = add_sub_two(num);
        (a - 1, b + 1)
    }

    fn add_sub_twenty(num: i32) -> (i32, i32) {
        let (a, b) = add_sub_ten(num);
        (a + 10, b - 10)
    }
}

impl exports::test::protocol::guest::Guest for GuestImpl {
    fn add_three(num: i32) -> i32 {
        let num = test::protocol::host::add_one(num);
        let num = test::protocol::host::add_one(num);
        let num = test::protocol::host::add_one(num);
        num
    }
}

export_test_world!(GuestImpl);
