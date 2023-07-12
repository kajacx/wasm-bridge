use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "primitives",
});

struct Imports;

impl PrimitivesImports for Imports {
    fn add_one_s8(&mut self, num: i8) -> Result<i8> {
        Ok(num + 1)
    }

    fn add_one_s16(&mut self, num: i16) -> Result<i16> {
        Ok(num + 1)
    }

    fn add_one_s32(&mut self, num: i32) -> Result<i32> {
        Ok(num + 1)
    }

    fn add_one_s64(&mut self, num: i64) -> Result<i64> {
        Ok(num + 1)
    }

    fn add_one_u8(&mut self, num: u8) -> Result<u8> {
        Ok(num + 1)
    }

    fn add_one_u16(&mut self, num: u16) -> Result<u16> {
        Ok(num + 1)
    }

    fn add_one_u32(&mut self, num: u32) -> Result<u32> {
        Ok(num + 1)
    }

    fn add_one_u64(&mut self, num: u64) -> Result<u64> {
        Ok(num + 1)
    }

    fn add_one_float32(&mut self, num: f32) -> Result<f32> {
        Ok(num + 1.0)
    }

    fn add_one_float64(&mut self, num: f64) -> Result<f64> {
        Ok(num + 1.0)
    }

    fn negate(&mut self, value: bool) -> Result<bool> {
        Ok(!value)
    }

    fn add_b(&mut self, text: String) -> Result<String> {
        Ok(format!("{text}b"))
    }
}

pub fn run_test(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, Imports);

    let component = Component::new(&store.engine(), &component_bytes)?;

    let mut linker = Linker::new(store.engine());
    Primitives::add_to_linker(&mut linker, |data| data)?;

    let (instance, _) = Primitives::instantiate(&mut store, &component, &linker)?;

    let result = instance.call_add_three_s8(&mut store, 5i8)?;
    assert_eq!(result, 5i8 + 3);

    let result = instance.call_add_three_s16(&mut store, 5i16)?;
    assert_eq!(result, 5i16 + 3);

    let result = instance.call_add_three_s32(&mut store, 5i32)?;
    assert_eq!(result, 5i32 + 3);

    let result = instance.call_add_three_s64(&mut store, 5i64)?;
    assert_eq!(result, 5i64 + 3);

    let result = instance.call_add_three_u8(&mut store, 5u8)?;
    assert_eq!(result, 5u8 + 3);

    let result = instance.call_add_three_u16(&mut store, 5u16)?;
    assert_eq!(result, 5u16 + 3);

    let result = instance.call_add_three_u32(&mut store, 5u32)?;
    assert_eq!(result, 5u32 + 3);

    let result = instance.call_add_three_u64(&mut store, 5u64)?;
    assert_eq!(result, 5u64 + 3);

    let result = instance.call_add_three_float32(&mut store, 5.5f32)?;
    assert_eq!(result, 5.5f32 + 3.0);

    let result = instance.call_add_three_float64(&mut store, 5.5f64)?;
    assert_eq!(result, 5.5f64 + 3.0);

    let result = instance.call_negate_times(&mut store, true, 0)?;
    assert_eq!(result, true);

    let result = instance.call_negate_times(&mut store, false, 3)?;
    assert_eq!(result, true);

    let result = instance.call_add_abc(&mut store, "hello ")?;
    assert_eq!(result, "hello abc");

    Ok(())
}
