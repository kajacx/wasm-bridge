use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "lists",
});

struct HostData;

impl ListsImports for HostData {
    fn push_bool(&mut self, mut bools: Vec<bool>, a: bool) -> Result<Vec<bool>> {
        bools.push(a);
        Ok(bools)
    }

    fn push_s8(&mut self, mut numbers: Vec<i8>, a: i8) -> Result<Vec<i8>> {
        numbers.push(a);
        Ok(numbers)
    }

    fn push_s16(&mut self, mut numbers: Vec<i16>, a: i16) -> Result<Vec<i16>> {
        numbers.push(a);
        Ok(numbers)
    }

    fn push_s32(&mut self, mut numbers: Vec<i32>, a: i32) -> Result<Vec<i32>> {
        numbers.push(a);
        Ok(numbers)
    }

    fn push_s64(&mut self, mut numbers: Vec<i64>, a: i64) -> Result<Vec<i64>> {
        numbers.push(a);
        Ok(numbers)
    }

    fn push_u8(&mut self, mut numbers: Vec<u8>, a: u8) -> Result<Vec<u8>> {
        numbers.push(a);
        Ok(numbers)
    }

    fn push_u16(&mut self, mut numbers: Vec<u16>, a: u16) -> Result<Vec<u16>> {
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

    fn push_char(&mut self, mut chars: Vec<char>, a: char) -> Result<Vec<char>> {
        chars.push(a);
        Ok(chars)
    }

    fn push_string(&mut self, mut numbers: Vec<String>, a: String) -> Result<Vec<String>> {
        numbers.push(a);
        Ok(numbers)
    }

    fn increment_bs(&mut self, mut variants: Vec<AbVariant>) -> Result<Vec<AbVariant>> {
        variants.iter_mut().for_each(|var| {
            *var = if let AbVariant::B(b) = var {
                AbVariant::B(*b + 1)
            } else {
                *var
            }
        });
        Ok(variants)
    }
}

pub fn run_test(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, HostData);

    let component = Component::new(&store.engine(), &component_bytes)?;

    let mut linker = Linker::new(store.engine());
    Lists::add_to_linker(&mut linker, |data| data)?;

    let (instance, _) = Lists::instantiate(&mut store, &component, &linker)?;

    let test_vec: Vec<_> = (0..100)
        .map(|i| {
            if i % 2 == 0 {
                AbVariant::A(i)
            } else {
                AbVariant::B(i)
            }
        })
        .collect();

    super::bench("list of variants", || {
        instance.call_increment_abs(&mut store, &test_vec)
    });

    Ok(())
}
