use std::sync::Mutex;

use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

use wasm_bridge_wasi::preview2::*;
use wasm_bridge_wasi::preview2::command;


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
    no_config(component_bytes).await.unwrap();
    custom_clock(component_bytes).await.unwrap();

    Ok(())
}

async fn no_config(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let table = Table::new();
    let wasi = WasiCtxBuilder::new().build();

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, State { table, wasi });

    let component = Component::new(&store.engine(), &component_bytes).unwrap();

    let mut linker = Linker::new(store.engine());
    command::add_to_linker(&mut linker).unwrap();

    let (instance, _) = Clock::instantiate_async(&mut store, &component, &linker).await.unwrap();

    let seconds_real = seconds_since_epoch();
    let seconds_guest = instance.call_seconds_since_epoch(&mut store).await.unwrap();
    panic!("SECONDS GUEST: {seconds_guest}");
    assert!(
        seconds_guest < seconds_real + 60 && seconds_guest > seconds_real - 60,
        "Guest should return time within one minute"
    );

    let bench = instance.call_nanoseconds_bench(&mut store).await.unwrap();
    assert!(
        bench > 1_000 && bench < 10_000_000_000,
        "bench should take between 1 microsecond and 10 seconds"
    );

    Ok(())
}

async fn custom_clock(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let table = Table::new();
    let wasi = WasiCtxBuilder::new()
        .wall_clock(FiveMinutesAfterEpoch)
        .monotonic_clock(FiveSecondsBetweenCalls(Mutex::new(0)))
        .build();

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, State { table, wasi });

    let component = Component::new(&store.engine(), &component_bytes).unwrap();

    let mut linker = Linker::new(store.engine());
    command::add_to_linker(&mut linker).unwrap();

    let (instance, _) = Clock::instantiate_async(&mut store, &component, &linker).await.unwrap();

    let seconds_real = 5 * 60; // 5 minutes
    let seconds_guest = instance.call_seconds_since_epoch(&mut store).await.unwrap();
    assert!(
        seconds_guest < seconds_real + 10 && seconds_guest > seconds_real - 10,
        "Guest should return time withing ten seconds"
    );

    let bench = instance.call_nanoseconds_bench(&mut store).await.unwrap();
    assert_eq!(bench, 5_000_000_000,
        "bench should think it took exactly 5 seconds"
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

struct FiveMinutesAfterEpoch;

impl HostWallClock for FiveMinutesAfterEpoch {
    fn now(&self) -> std::time::Duration {
        std::time::Duration::from_secs(5 * 60)
    }

    fn resolution(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(1)
    }
}

struct FiveSecondsBetweenCalls(Mutex<u64>);

impl HostMonotonicClock for FiveSecondsBetweenCalls {
    fn now(&self) -> u64 {
        let mut lock = self.0.try_lock().unwrap();
        *lock = *lock + 5_000_000_000;
        *lock
    }

    fn resolution(&self) -> u64 {
        1
    }
}
