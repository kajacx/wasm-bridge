use wasm_bridge::{
    component::{Linker, new_component_async},
    Config, Engine, Result, Store,
};

use wasm_bridge_wasi::preview2::*;

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "wit-imports",
    async: {
        only_imports: [],
    },
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

impl WitImportsImports for State {
    fn add_one(&mut self, num: i32) -> Result<i32> {
        Ok(num + 1)
    }

    fn push_string(&mut self, mut strings: Vec<String>, a: String) -> Result<Vec<String>> {
        strings.push(a);
        Ok(strings)
    }
}

pub async fn run_test(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let table = Table::new();
    let wasi = WasiCtxBuilder::new().build();

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, State { table, wasi });

    let component = new_component_async(&store.engine(), &component_bytes).await?;

    let mut linker = Linker::new(store.engine());
    command::add_to_linker(&mut linker)?;
    WitImports::add_to_linker(&mut linker, |data| data)?;

    let (instance, _) = WitImports::instantiate_async(&mut store, &component, &linker).await?;

    let result = instance.call_add_three(&mut store, 5).await?;
    assert_eq!(result, 8);

    let result = instance.call_push_strings(&mut store, &["a".into(), "b".into()], "c", "d").await?;
    assert_eq!(result, vec!["a", "b", "c", "d"]);

    Ok(())
}
