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
    fn create_player(
        &mut self,
        name: String,
        inventory: Vec<String>,
        counts: Vec<u32>,
    ) -> Result<Player> {
        Ok(Player {
            name,
            inventory,
            counts,
        })
    }
}

#[test]
#[wasm_bindgen_test]
fn records() {
    wit_components_tests::setup_tracing();
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, Host);

    let component = Component::new(store.engine(), GUEST_BYTES).unwrap();

    let mut linker = Linker::new(store.engine());
    Records::add_to_linker(&mut linker, |data| data).unwrap();

    let (instance, _) = Records::instantiate(&mut store, &component, &linker).unwrap();

    let result = instance.call_get_inventory(&mut store).unwrap();

    assert_eq!(result, ["sword", "shield", "apple"]);
    let result = instance.call_get_counts(&mut store).unwrap();
    assert_eq!(result, [1, 2, 5]);
}

const GUEST_BYTES: &[u8] =
    include_bytes!("../../../target/wasm32-unknown-unknown/debug/records_guest.wasm");
