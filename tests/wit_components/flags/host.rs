use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "flags-test",
});

struct Host;

impl FlagsTestImports for Host {
    fn import_add_green(&mut self, colors: Colors) -> Result<Colors> {
        Ok(colors | Colors::GREEN)
    }

    fn import_push_green(&mut self, mut colors: Vec<Colors>) -> Result<Vec<Colors>> {
        colors.push(Colors::GREEN);
        Ok(colors)
    }

    fn import_add_first(&mut self, values: ManyFlags) -> Result<ManyFlags> {
        Ok(values | ManyFlags::FLAG01)
    }

    fn import_push_first(&mut self, mut values: Vec<ManyFlags>) -> Result<Vec<ManyFlags>> {
        values.push(ManyFlags::FLAG01);
        Ok(values)
    }
}

pub fn run_test(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, Host);

    let component = Component::new(&store.engine(), &component_bytes).unwrap();

    let mut linker = Linker::new(store.engine());
    FlagsTest::add_to_linker(&mut linker, |data| data).unwrap();

    let (instance, _) = FlagsTest::instantiate(&mut store, &component, &linker).unwrap();

    let result = instance
        .call_export_add_green_and_blue(&mut store, Colors::RED)
        .unwrap();
    assert_eq!(result, Colors::all());

    let result = instance
        .call_export_push_green_and_blue(&mut store, &[Colors::RED])
        .unwrap();
    assert_eq!(result, vec![Colors::RED, Colors::GREEN, Colors::BLUE]);

    let flags56 = ManyFlags::FLAG05 | ManyFlags::FLAG06;
    let result = instance
        .call_export_add_first_and_last(&mut store, flags56)
        .unwrap();
    assert_eq!(result, flags56 | ManyFlags::FLAG01 | ManyFlags::FLAG39);

    let result = instance
        .call_export_push_first_and_last(&mut store, &[flags56])
        .unwrap();
    assert_eq!(result, vec![flags56, ManyFlags::FLAG01, ManyFlags::FLAG39]);

    Ok(())
}
