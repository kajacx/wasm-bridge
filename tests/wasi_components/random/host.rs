use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

use wasm_bridge_wasi::*;

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "random",
    async: true,
});

struct State {
    table: ResourceTable,
    wasi: WasiCtx,
}

impl WasiView for State {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

pub async fn run_test(component_bytes: &[u8]) -> Result<()> {
    default_random(component_bytes).await.unwrap();
    custom_random(component_bytes).await.unwrap();

    Ok(())
}

async fn default_random(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let table = ResourceTable::new();
    let wasi = WasiCtxBuilder::new().build();

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, State { table, wasi });

    #[allow(deprecated)]
    let component = Component::new(&store.engine(), &component_bytes).unwrap(); 

    let mut linker = Linker::new(store.engine());
    command::add_to_linker(&mut linker).unwrap();

    let (instance, _) = Random::instantiate_async(&mut store, &component, &linker).await.unwrap();
    let number1 = instance.call_random_number(&mut store).await.unwrap();
    let bytes1 = instance.call_random_bytes(&mut store).await.unwrap();

    let (instance, _) = Random::instantiate_async(&mut store, &component, &linker).await.unwrap();
    let number2 = instance.call_random_number(&mut store).await.unwrap();
    let bytes2 = instance.call_random_bytes(&mut store).await.unwrap();

    assert_ne!(number1, number2, "Two random u64 should not really be equal");
    assert_ne!(bytes1, bytes2, "32 random bytes should definitely noy be equal");

    Ok(())
}

async fn custom_random(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let table = ResourceTable::new();
    let wasi = WasiCtxBuilder::new()
        .secure_random(ZeroRng)
        .build();

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, State { table, wasi });

    #[allow(deprecated)]
    let component = Component::new(&store.engine(), &component_bytes).unwrap(); 

    let mut linker = Linker::new(store.engine());
    command::add_to_linker(&mut linker).unwrap();

    let (instance, _) = Random::instantiate_async(&mut store, &component, &linker).await.unwrap();
    let number1 = instance.call_random_number(&mut store).await.unwrap();
    let bytes1 = instance.call_random_bytes(&mut store).await.unwrap();

    let (instance, _) = Random::instantiate_async(&mut store, &component, &linker).await.unwrap();
    let number2 = instance.call_random_number(&mut store).await.unwrap();
    let bytes2 = instance.call_random_bytes(&mut store).await.unwrap();

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
