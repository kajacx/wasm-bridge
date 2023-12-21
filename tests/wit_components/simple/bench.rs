use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "simple",
});

pub fn run_test(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, ());

    let component = Component::new(&store.engine(), &component_bytes)?;

    let linker = Linker::new(store.engine());

    let (instance, _) = Simple::instantiate(&mut store, &component, &linker)?;

    let big_vec: Vec<_> = (0..1000).into_iter().collect();

    super::bench("Call exported methods", || {
        let result = instance.call_push_s32s(&mut store, &big_vec, 3, 4).unwrap();
        assert_eq!(result.len(), 1002);

        let result = instance
            .call_push_u32s(&mut store, &[10, u32::MAX - 10], 3, 4)
            .unwrap();
        assert_eq!(result, vec![10, u32::MAX - 10, 3, 4]);
    });

    Ok(())
}
