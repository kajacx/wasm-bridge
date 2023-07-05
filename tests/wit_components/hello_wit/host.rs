use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

mod wasmtime {
    pub use wasm_bridge::*;
}

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "test-world"
});

pub fn run_test(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, ());

    let component = Component::new(&store.engine(), &component_bytes)?;

    let linker = Linker::new(store.engine());

    let (instance, _) = TestWorld::instantiate(&mut store, &component, &linker)?;

    let result = instance.call_add_hello(&mut store, "world".into())?;
    assert_eq!(result, "Hello world");

    Ok(())
}
