use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

use wasm_bridge::wasi::preview2::*;

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "test-world",
    async: true,
    with: {
       "wasi:cli-base/stdin": wasi::cli_base::stdin,
       "wasi:cli-base/stdout": wasi::cli_base::stdout,
       "wasi:cli-base/stderr": wasi::cli_base::stderr,
    }
});

struct State {
    table: Table,
    wasi: WasiCtx,
}

impl WasiView for State {
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

#[async_trait::async_trait]
impl TestWorldImports for State {
    async fn add_one(&mut self, num: i32) -> Result<i32> {
        Ok(num + 1)
    }
}

pub async fn run_test(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let mut table = Table::new();
    let wasi = WasiCtxBuilder::new().build(&mut table)?;

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, State { table, wasi });

    let component = Component::new(&store.engine(), &component_bytes)?;

    let mut linker = Linker::new(store.engine());
    TestWorld::add_to_linker(&mut linker, |data| data)?;
    wasi::command::add_to_linker(&mut linker)?;

    let (instance, _) = TestWorld::instantiate_async(&mut store, &component, &linker).await?;

    let result = instance.call_add_three(&mut store, 5).await?;
    assert_eq!(result, 8);

    Ok(())
}
