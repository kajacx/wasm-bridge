use std::error::Error;
use wasm_bridge::*;

pub fn run_test(bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut store = Store::<()>::default();

    let module = Module::new(&store.engine(), bytes)?;

    let mut linker = Linker::new(store.engine());
    linker.func_wrap("imported_fns", "add_one_i32", |_: Caller<()>, val: i32| {
        val.wrapping_add(1)
    })?;
    linker.func_wrap("imported_fns", "add_one_i64", |_: Caller<()>, val: i64| {
        val.wrapping_add(1)
    })?;
    linker.func_wrap("imported_fns", "add_one_f32", |_: Caller<()>, val: f32| {
        val + 1.0
    })?;
    linker.func_wrap("imported_fns", "add_one_f64", |_: Caller<()>, val: f64| {
        val + 1.0
    })?;
    let instance = linker.instantiate(&mut store, &module)?;

    // Signed integers
    let add_three_i32 = instance.get_typed_func::<i32, i32>(&mut store, "add_three_i32")?;

    for number in [-10, -1, 0, 10, i32::MIN + 1, i32::MAX - 2] {
        let returned = add_three_i32.call(&mut store, number)?;
        assert_eq!(returned, number.wrapping_add(3));
    }

    let add_three_i64 = instance.get_typed_func::<i64, i64>(&mut store, "add_three_i64")?;

    for number in [-10, -1, 0, 10, i64::MIN + 1, i64::MAX - 2] {
        let returned = add_three_i64.call(&mut store, number)?;
        assert_eq!(returned, number.wrapping_add(3));
    }

    // Unsigned integers
    let add_three_u32 = instance.get_typed_func::<u32, u32>(&mut store, "add_three_i32")?;

    for number in [0, 10, u32::MAX / 2 - 1, u32::MAX - 2] {
        let returned = add_three_u32.call(&mut store, number)?;
        assert_eq!(returned, number.wrapping_add(3));
    }

    let add_three_u64 = instance.get_typed_func::<u64, u64>(&mut store, "add_three_i64")?;

    for number in [0, 10, u64::MAX / 2 - 1, u64::MAX - 2] {
        let returned = add_three_u64.call(&mut store, number)?;
        assert_eq!(returned, number.wrapping_add(3));
    }

    // Floats
    let add_three_f32 = instance.get_typed_func::<f32, f32>(&mut store, "add_three_f32")?;

    for number in [0.0, 10.25, -2.5, 1_000_000.5, -1_000_000.5] {
        let returned = add_three_f32.call(&mut store, number)?;
        assert_eq!(returned, number + 3.0);
    }

    let add_three_f64 = instance.get_typed_func::<f64, f64>(&mut store, "add_three_f64")?;

    for number in [0.0, 10.25, -2.5, 10_000_000_000.5, -10_000_000_000.5] {
        let returned = add_three_f64.call(&mut store, number)?;
        assert_eq!(returned, number + 3.0);
    }

    Ok(())
}
