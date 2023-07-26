use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

use wasm_bridge::wasi::preview2::*;


wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "clock",
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

pub async fn run_test(component_bytes: &[u8]) -> Result<()> {
    no_config(component_bytes).await?;

    Ok(())
}

async fn no_config(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let mut table = Table::new();
    let wasi = WasiCtxBuilder::new().build(&mut table)?;

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, State { table, wasi });

    let component = Component::new(&store.engine(), &component_bytes)?;

    let mut linker = Linker::new(store.engine());
    wasi::command::add_to_linker(&mut linker)?;

    let (instance, _) = Clock::instantiate_async(&mut store, &component, &linker).await?;

    let seconds_real = seconds_since_epoch();
    let seconds_guest = instance.call_seconds_since_epoch(&mut store).await?;
    assert!(
        seconds_guest < seconds_real + 60 && seconds_guest > seconds_real - 60,
        "Guest should return time withing one minute"
    );

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn seconds_since_epoch() -> u64 {
    let now = std::time::SystemTime::now();
    let interval = now
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap();
    interval.as_secs()
} 

#[cfg(target_arch = "wasm32")]
fn seconds_since_epoch() -> u64 {
    let value = wasm_bridge::js_sys::eval("Math.round(Date.now() / 1000)").unwrap();
    value.as_f64().unwrap() as _
}