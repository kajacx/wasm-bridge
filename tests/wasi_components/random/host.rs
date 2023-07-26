use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

use wasm_bridge::wasi::preview2::*;

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "random",
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
    default_random(component_bytes).await?;
    custom_random(component_bytes).await?;

    Ok(())
}

async fn default_random(component_bytes: &[u8]) -> Result<()> {
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

    let (instance, _) = Random::instantiate_async(&mut store, &component, &linker).await?;
    let number1 = instance.call_random_number(&mut store).await?;
    let bytes1 = instance.call_random_bytes(&mut store).await?;

    let (instance, _) = Random::instantiate_async(&mut store, &component, &linker).await?;
    let number2 = instance.call_random_number(&mut store).await?;
    let bytes2 = instance.call_random_bytes(&mut store).await?;

    assert_ne!(number1, number2, "Two random u64 should not really be equal");
    assert_ne!(bytes1, bytes2, "32 random bytes should definitely noy be equal");

    Ok(())
}

async fn custom_random(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let mut table = Table::new();
    let wasi = WasiCtxBuilder::new()
        .set_secure_random_to_custom_generator(ZeroRng)
        .build(&mut table)?;

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, State { table, wasi });

    let component = Component::new(&store.engine(), &component_bytes)?;

    let mut linker = Linker::new(store.engine());
    wasi::command::add_to_linker(&mut linker)?;

    let (instance, _) = Random::instantiate_async(&mut store, &component, &linker).await?;
    let number1 = instance.call_random_number(&mut store).await?;
    let bytes1 = instance.call_random_bytes(&mut store).await?;

    let (instance, _) = Random::instantiate_async(&mut store, &component, &linker).await?;
    let number2 = instance.call_random_number(&mut store).await?;
    let bytes2 = instance.call_random_bytes(&mut store).await?;

    assert_eq!(number1, number2);
    assert_eq!(bytes1, bytes2);

    Ok(())
}

struct ZeroRng;

impl rand_core::RngCore for ZeroRng {
    fn next_u32(&mut self) -> u32 {
        0
    }

    fn next_u64(&mut self) -> u64 {
        0
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}
