use std::error::Error;
use wasm_bridge::*;

pub fn run_test(bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut store = Store::<()>::default();

    let module = Module::new(&store.engine(), bytes)?;

    let instance = Instance::new(&mut store, &module, &[])?;

    let rate_number = instance.get_typed_func::<i32, i32>(&mut store, "add_five_i32")?;

    for number in [-10, -1, 0, 10, i32::MIN + 1, i32::MAX - 2] {
        let returned = rate_number.call(&mut store, number)?;
        assert_eq!(returned, number.wrapping_add(5));
    }

    Ok(())
}
