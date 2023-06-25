use std::error::Error;
use wasm_bridge::*;

pub fn run_test(bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut store = Store::<()>::default();
    let module = Module::new(&store.engine(), bytes)?;
    let instance = Instance::new(&mut store, &module, &[])?;

    single_value(&mut store, &instance)?;
    few_values(&mut store, &instance)?;
    errors(bytes)?;

    Ok(())
}

fn single_value(mut store: &mut Store<()>, instance: &Instance) -> Result<(), Box<dyn Error>> {
    // Signed integers
    let add_five_i32 = instance.get_typed_func::<i32, i32>(&mut store, "add_five_i32")?;

    for number in [-10, -1, 0, 10, i32::MIN + 1, i32::MAX - 2] {
        let returned = add_five_i32.call(&mut store, number)?;
        assert_eq!(returned, number.wrapping_add(5));
    }

    let add_five_i64 = instance.get_typed_func::<i64, i64>(&mut store, "add_five_i64")?;

    for number in [-10, -1, 0, 10, i64::MIN + 1, i64::MAX - 2] {
        let returned = add_five_i64.call(&mut store, number)?;
        assert_eq!(returned, number.wrapping_add(5));
    }

    // Unsigned integers
    let add_five_u32 = instance.get_typed_func::<u32, u32>(&mut store, "add_five_i32")?;

    for number in [0, 10, u32::MAX / 2 - 1, u32::MAX - 2] {
        let returned = add_five_u32.call(&mut store, number)?;
        assert_eq!(returned, number.wrapping_add(5));
    }

    let add_five_u64 = instance.get_typed_func::<u64, u64>(&mut store, "add_five_i64")?;

    for number in [0, 10, u64::MAX / 2 - 1, u64::MAX - 2] {
        let returned = add_five_u64.call(&mut store, number)?;
        assert_eq!(returned, number.wrapping_add(5));
    }

    // Floats
    let add_five_f32 = instance.get_typed_func::<f32, f32>(&mut store, "add_five_f32")?;

    for number in [0.0, 10.25, -2.5, 1_000_000.5, -1_000_000.5] {
        let returned = add_five_f32.call(&mut store, number)?;
        assert_eq!(returned, number + 5.0);
    }

    let add_five_f64 = instance.get_typed_func::<f64, f64>(&mut store, "add_five_f64")?;

    for number in [0.0, 10.25, -2.5, 10_000_000_000.5, -10_000_000_000.5] {
        let returned = add_five_f64.call(&mut store, number)?;
        assert_eq!(returned, number + 5.0);
    }

    Ok(())
}

fn few_values(mut store: &mut Store<()>, instance: &Instance) -> Result<(), Box<dyn Error>> {
    // Multiple arguments
    let add_i32 = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add_i32")?;

    let returned = add_i32.call(&mut store, (5, 10))?;
    assert_eq!(returned, 5 + 10);

    // Multiple results
    let add_sub_ten_i32 =
        instance.get_typed_func::<i32, (i32, i32)>(&mut store, "add_sub_ten_i32")?;

    let (a, b) = add_sub_ten_i32.call(&mut store, 50)?;
    assert_eq!(a, 50i32 + 10);
    assert_eq!(b, 50i32 - 10);

    // Single-element tuple
    let add_five_f64 = instance.get_typed_func::<(f64,), (f64,)>(&mut store, "add_five_f64")?;
    let returned = add_five_f64.call(&mut store, (5.5,))?;
    assert_eq!(returned, (5.5 + 5.0,));

    Ok(())
}

fn errors(bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut store = Store::<()>::default();

    // Bad binary bytes
    Module::new(&store.engine(), &[1, 5])
        .map(|_| ())
        .expect_err("should not create module");

    // Bad text bytes
    Module::new(&store.engine(), "not a valit wat module".as_bytes())
        .map(|_| ())
        .expect_err("should not create module");

    let module = Module::new(&store.engine(), bytes)?;
    let instance = Instance::new(&mut store, &module, &[])?;

    // Non-existing function
    instance
        .get_typed_func::<i32, i32>(&mut store, "non_existing")
        .map(|_| ())
        .expect_err("should not get function");

    // TODO: Number of arguments in currently the only type info avaliable
    // Maybe look into how wasmer does it?
    // Bad number of arguments
    instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "add_five_i32")
        .map(|_| ())
        .expect_err("should not get function");

    let panics = instance.get_typed_func::<(), ()>(&mut store, "panics")?;

    // Implementation panics
    panics
        .call(&mut store, ())
        .expect_err("should not get result");

    Ok(())
}
