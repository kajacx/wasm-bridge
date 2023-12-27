use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "tuples",
});

struct Imports;

impl TuplesImports for Imports {
    fn add_sub_two(&mut self, num: i32) -> Result<(i32, i32)> {
        Ok((num + 2, num - 2))
    }

    fn add_sub_ten(&mut self, num: i32) -> Result<(i32, i32)> {
        Ok((num + 10, num - 10))
    }
}

pub fn run_test(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, Imports);

    let component = Component::new(&store.engine(), &component_bytes)?;

    let mut linker = Linker::new(store.engine());
    Tuples::add_to_linker(&mut linker, |data| data)?;

    let (instance, _) = Tuples::instantiate(&mut store, &component, &linker)?;

    let result = instance.call_add_sub_one(&mut store, 5)?;
    assert_eq!(result, (6, 4));

    let result = instance.call_add_sub_twenty(&mut store, 5)?;
    assert_eq!(result, (25, -15));

    Ok(())
}
