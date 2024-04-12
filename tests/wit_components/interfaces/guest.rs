wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "interfaces",
});

struct GuestImpl;

impl Guest for GuestImpl {
    fn normal_export(n: i32) -> i32 {
        normal_import(n)
    }
}

impl exports::component_test::wit_protocol::guest_add::Guest for GuestImpl {
    fn add_three(num: i32) -> i32 {
        let num = component_test::wit_protocol::host_add::add_one(num);
        let num = component_test::wit_protocol::host_add::add_one(num);
        let num = component_test::wit_protocol::host_add::add_one(num);
        num
    }
}

impl exports::guest_sub::Guest for GuestImpl {
    fn sub_three(num: i32) -> i32 {
        let num = host_sub::sub_one(num);
        let num = host_sub::sub_one(num);
        let num = host_sub::sub_one(num);
        num
    }
}

export!(GuestImpl);
