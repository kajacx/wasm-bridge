use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

use wasm_bridge_wasi::preview2::command;
use wasm_bridge_wasi::preview2::*;

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "environment",
    async: true,
});

struct State {
    table: Table,
    wasi: WasiCtx,
}

impl WasiView for State {
    fn table(&mut self) -> &mut Table {
        &mut self.table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

pub async fn run_test(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let table = Table::new();
    let wasi = WasiCtxBuilder::new()
        .env("env0", "val0")
        .envs(&[("env1", "val1"), ("env2", "val2")])
        .build();

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, State { table, wasi });

    let component = Component::new(&store.engine(), &component_bytes).unwrap();

    let mut linker = Linker::new(store.engine());
    command::add_to_linker(&mut linker).unwrap();

    let (instance, _) = Environment::instantiate_async(&mut store, &component, &linker)
        .await
        .unwrap();

    let result = instance
        .call_get_env_var(&mut store, &"env0")
        .await
        .unwrap();
    assert_eq!(result, Some("val0".to_string()));

    let result = instance
        .call_get_env_var(&mut store, &"env1")
        .await
        .unwrap();
    assert_eq!(result, Some("val1".to_string()));

    let result = instance
        .call_get_env_var(&mut store, &"env2")
        .await
        .unwrap();
    assert_eq!(result, Some("val2".to_string()));

    let result = instance
        .call_get_env_var(&mut store, &"env3")
        .await
        .unwrap();
    assert_eq!(result, None);

    Ok(())
}
