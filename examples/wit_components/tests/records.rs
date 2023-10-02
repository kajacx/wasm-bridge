use wasm_bindgen_test::wasm_bindgen_test;
use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

wasm_bridge::component::bindgen!({
    path: "./records.wit",
    world: "records",
});

struct Host;
impl RecordsImports for Host {
    fn create_player(&mut self, name: String, inventory: Vec<u32>) -> Result<Player> {
        Ok(Player { name, inventory })
    }
}

#[test]
#[wasm_bindgen_test]
fn records() {
    use tracing_subscriber::prelude::*;
    #[cfg(target_arch = "wasm32")]
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true) // Only partially supported across browsers
        .without_time()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
        .with_writer(tracing_web::MakeConsoleWriter); // write events to the console

    #[cfg(not(target_arch = "wasm32"))]
    let fmt_layer = tracing_subscriber::fmt::layer().with_ansi(true);

    tracing_subscriber::registry().with(fmt_layer).init();

    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, Host);

    let component = Component::new(store.engine(), GUEST_BYTES).unwrap();

    let mut linker = Linker::new(store.engine());
    Records::add_to_linker(&mut linker, |data| data).unwrap();

    let (instance, _) = Records::instantiate(&mut store, &component, &linker).unwrap();

    let result = instance
        .call_get_inventory(
            &mut store,
            &Player {
                name: "Foo".into(),
                inventory: vec![2, 6, 7],
            },
        )
        .unwrap();

    assert_eq!(result, vec![2, 6, 7]);
}

const GUEST_BYTES: &[u8] =
    include_bytes!("../../../target/wasm32-unknown-unknown/debug/records_guest.wasm");
