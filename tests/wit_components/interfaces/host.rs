use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "interfaces",
});

struct HostData;

impl InterfacesImports for HostData {
    fn normal_import(&mut self, num: i32) -> Result<i32> {
        Ok(num)
    }
}

impl component_test::wit_protocol::host_add::Host for HostData {
    fn add_one(&mut self, num: i32) -> Result<i32> {
        Ok(num + 1)
    }
}

impl host_sub::Host for HostData {
    fn sub_one(&mut self, num: i32) -> Result<i32> {
        Ok(num - 1)
    }
}

pub fn run_test(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, HostData);

    #[allow(deprecated)]
    let component = Component::new(&store.engine(), &component_bytes).unwrap();

    let mut linker = Linker::new(store.engine());
    Interfaces::add_to_linker(&mut linker, |data| data).unwrap();

    #[allow(deprecated)]
    let (instance, _) = Interfaces::instantiate(&mut store, &component, &linker).unwrap();

    let result = instance.call_normal_export(&mut store, 7).unwrap();
    assert_eq!(result, 7);

    let result = instance
        .component_test_wit_protocol_guest_add()
        .call_add_three(&mut store, 5)
        .unwrap();
    assert_eq!(result, 8);

    let result = instance.guest_sub().call_sub_three(&mut store, 5).unwrap();
    assert_eq!(result, 2);

    Ok(())
}
