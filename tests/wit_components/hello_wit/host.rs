use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

mod wasmtime {
    pub use wasm_bridge::*;
}

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "test-world",
});

struct HostData {}

impl TestWorldImports for HostData {
    fn add_b(&mut self, text: String) -> Result<String> {
        Ok(text + "b")
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

    let result = instance.call_add_hello(&mut store, "world")?;
    assert_eq!(result, "Hello world");

    let result = instance.call_add_abc(&mut store, "Hello ")?;
    assert_eq!(result, "Hello abc");

    Ok(())
}
