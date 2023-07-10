use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "test-world",
});

struct HostData {}

impl TestWorldImports for HostData {
    fn push_s32(&mut self, mut numbers: Vec<i32>, a: i32) -> Result<Vec<i32>> {
        numbers.push(a);
        Ok(numbers)
    }

    fn push_s64(&mut self, mut numbers: Vec<i64>, a: i64) -> Result<Vec<i64>> {
        numbers.push(a);
        Ok(numbers)
    }

    fn push_u32(&mut self, mut numbers: Vec<u32>, a: u32) -> Result<Vec<u32>> {
        numbers.push(a);
        Ok(numbers)
    }

    fn push_u64(&mut self, mut numbers: Vec<u64>, a: u64) -> Result<Vec<u64>> {
        numbers.push(a);
        Ok(numbers)
    }

    fn push_float32(&mut self, mut numbers: Vec<f32>, a: f32) -> Result<Vec<f32>> {
        numbers.push(a);
        Ok(numbers)
    }

    fn push_float64(&mut self, mut numbers: Vec<f64>, a: f64) -> Result<Vec<f64>> {
        numbers.push(a);
        Ok(numbers)
    }

    fn push_string(&mut self, mut numbers: Vec<String>, a: String) -> Result<Vec<String>> {
        numbers.push(a);
        Ok(numbers)
    }
}

pub fn run_test(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, HostData {});

    let component = Component::new(&store.engine(), &component_bytes)?;

    let mut linker = Linker::new(store.engine());
    TestWorld::add_to_linker(&mut linker, |data| data)?;

    let (instance, _) = TestWorld::instantiate(&mut store, &component, &linker)?;

    let result = instance.call_push_s32s(&mut store, &[-10, 200], 3, 4)?;
    assert_eq!(result, vec![-10, 200, 3, 4]);

    let result = instance.call_push_s64s(&mut store, &[-10_000_000_000, 200_000_000_000], 3, 4)?;
    assert_eq!(result, vec![-10_000_000_000, 200_000_000_000, 3, 4]);

    let result = instance.call_push_u32s(&mut store, &[10, u32::MAX - 10], 3, 4)?;
    assert_eq!(result, vec![10, u32::MAX - 10, 3, 4]);

    let result = instance.call_push_u64s(&mut store, &[10, u64::MAX - 10], 3, 4)?;
    assert_eq!(result, vec![10, u64::MAX - 10, 3, 4]);

    let result = instance.call_push_float32s(&mut store, &[5.5, -10.25], 3.0, 4.0)?;
    assert_eq!(result, vec![5.5, -10.25, 3.0, 4.0]);

    let result = instance.call_push_float64s(&mut store, &[5.5, -10.25], 3.0, 4.0)?;
    assert_eq!(result, vec![5.5, -10.25, 3.0, 4.0]);

    let result = instance.call_push_strings(&mut store, &["hello", "world"], "three", "four")?;
    assert_eq!(
        result,
        vec![
            "hello".to_owned(),
            "world".to_owned(),
            "three".to_owned(),
            "four".to_owned()
        ]
    );

    Ok(())
}
