use std::error::Error;
use wasm_bridge::*;

pub fn run_test(bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut store = Store::<()>::default();

    // Test that creating a new module from bad bytes fails correctly
    Module::new(&store.engine(), &[]).expect_err("should not create module");

    let module = Module::new(&store.engine(), bytes)?;

    // Creating a new instance can fail because of bad imports, so nothing to test yet.
    let instance = Instance::new(&mut store, &module, &[])?;

    // Getting a non-existing function should return error
    instance
        .get_typed_func::<i32, i32>(&mut store, "non-existing")
        .expect_err("should not get function");

    let add_five_i32 = instance.get_typed_func::<i32, i32>(&mut store, "add_five_i32")?;

    // Implementation panics
    add_five_i32
        .call(&mut store, 10)
        .expect_err("should not call function");

    Ok(())
}
