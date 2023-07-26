use wasm_bridge::{
    component::{Linker, component_new_async},
    Config, Engine, Result, Store,
};

use wasm_bridge::wasi::preview2::*;

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "wit-imports",
    async: true,
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

#[wasm_bridge::async_trait]
impl WitImportsImports for State {
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

    let component = component_new_async(&store.engine(), &component_bytes).await?;

    let mut linker = Linker::new(store.engine());
    wasi::command::add_to_linker(&mut linker)?;
    WitImports::add_to_linker(&mut linker, |data| data)?;

    let (instance, _) = WitImports::instantiate_async(&mut store, &component, &linker).await?;

    let result = instance.call_add_three(&mut store, 5).await?;
    assert_eq!(result, 8);

    Ok(())
}
