use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "test-world",
});

struct HostData {}

impl TestWorldImports for HostData {
    fn push_s32(&mut self, mut numbers: Vec<i32>, a: i32) -> Result<Vec<i32>> {
        numbers.push(a);
        Ok(numbers)
    }
}

pub fn run_test(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, HostData {});

    let component = Component::new(&store.engine(), &component_bytes)?;

    let mut linker = Linker::new(store.engine());
    TestWorld::add_to_linker(&mut linker, |data| data)?;

    let (instance, _) = TestWorld::instantiate(&mut store, &component, &linker)?;

    let result = instance.call_push_s32s(&mut store, &[-10, 200], 3, 4)?;
    assert_eq!(result, vec![-10, 200, 3, 4]);

    Ok(())
}
