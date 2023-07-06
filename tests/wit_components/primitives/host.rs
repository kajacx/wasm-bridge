use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

mod wasmtime {
    pub use wasm_bridge::*;
}

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "primitives",
});

pub fn run_test(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, ());

    let component = Component::new(&store.engine(), &component_bytes)?;

    let linker = Linker::new(store.engine());

    let (instance, _) = Primitives::instantiate(&mut store, &component, &linker)?;

    let result = instance.call_add_three_s32(&mut store, 5i32)?;
    assert_eq!(result, 5 + 3);

    let result = instance.call_add_three_s64(&mut store, 5i64)?;
    assert_eq!(result, 5 + 3);

    let result = instance.call_add_three_u32(&mut store, 5u32)?;
    assert_eq!(result, 5 + 3);

    let result = instance.call_add_three_u64(&mut store, 5u64)?;
    assert_eq!(result, 5 + 3);

    let result = instance.call_add_three_float32(&mut store, 5.5f32)?;
    assert_eq!(result, 5.5 + 3.0);

    let result = instance.call_add_three_float64(&mut store, 5.5f64)?;
    assert_eq!(result, 5.5 + 3.0);

    let result = instance.call_add_abc(&mut store, "hello ")?;
    assert_eq!(result, "hello abc");

    Ok(())
}
