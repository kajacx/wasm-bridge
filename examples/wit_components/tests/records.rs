use wasm_bindgen_test::wasm_bindgen_test;
use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

wasm_bridge::component::bindgen!({
    path: "./records.wit",
    world: "records",
});

#[derive(Default, Debug, Clone)]
struct Host {
    messages: Vec<Item>,
}

impl RecordsImports for Host {
    fn send_item(&mut self, item: Item) -> wasm_bridge::Result<()> {
        self.messages.push(item);

        Ok(())
    }

    // fn send_items(&mut self, items: Vec<Item>) -> wasm_bridge::Result<()> {
    //     self.messages.extend(items);

    //     Ok(())
    // }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}

#[test]
#[wasm_bindgen_test]
fn records() {
    wit_components_tests::setup_tracing();
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, Host::default());

    let component = Component::new(store.engine(), GUEST_BYTES).unwrap();

    let mut linker = Linker::new(store.engine());
    Records::add_to_linker(&mut linker, |data| data).unwrap();

    let (instance, _) = Records::instantiate(&mut store, &component, &linker).unwrap();

    instance.call_run(&mut store, &[]);

    let data = store.data();
    assert_eq!(data.messages, &[Item { a: 1, b: 2 }]);

    // assert_eq!(result, ["sword", "shield", "apple"]);
    // let result = instance.call_get_counts(&mut store).unwrap();
    // assert_eq!(result, [1, 2, 5]);
}

const GUEST_BYTES: &[u8] =
    include_bytes!("../../../target/wasm32-unknown-unknown/debug/records_guest.wasm");
