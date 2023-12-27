wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "interfaces",
    exports: {
        "component-test:wit-protocol/guest-add": GuestImplAdd,
        "guest-sub": GuestImplSub,
    }
});

struct GuestImplAdd;

impl exports::component_test::wit_protocol::guest_add::Guest for GuestImplAdd {
    fn add_three(num: i32) -> i32 {
        let num = component_test::wit_protocol::host_add::add_one(num);
        let num = component_test::wit_protocol::host_add::add_one(num);
        let num = component_test::wit_protocol::host_add::add_one(num);
        num
    }
}

struct GuestImplSub;

impl exports::guest_sub::Guest for GuestImplSub {
    fn sub_three(num: i32) -> i32 {
        let num = host_sub::sub_one(num);
        let num = host_sub::sub_one(num);
        let num = host_sub::sub_one(num);
        num
    }
}
