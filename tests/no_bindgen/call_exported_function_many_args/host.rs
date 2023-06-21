use std::error::Error;
use wasm_bridge::*;

pub fn run_test(bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut store = Store::<()>::default();

    let module = Module::new(&store.engine(), bytes)?;

    let instance = Instance::new(&mut store, &module, &[])?;

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

    // Multiple results, 64 bits
    let add_sub_ten_i64 =
        instance.get_typed_func::<i64, (i64, i64)>(&mut store, "add_sub_ten_i64")?;

    let (a, b) = add_sub_ten_i64.call(&mut store, 80)?;
    assert_eq!(a, 80i64 + 10);
    assert_eq!(b, 80i64 - 10);

    // Many different arguments
    let add_all =
        instance.get_typed_func::<(i32, i64, u32, u64, f32, f64), f64>(&mut store, "add_all")?;

    let result = add_all.call(&mut store, (10, 20, 30, 40, 50.25, 60.5))?;
    assert_eq!(result, 10.0 + 20.0 + 30.0 + 40.0 + 50.25 + 60.5);

    // Many arguments and results
    // TODO: Compilation to WASM doesn't work, error in on Rust's end.
    // let add_ten_i32s = instance
    //     .get_typed_func::<(i32, i32, i32, i32), (i32, i32, i32, i32)>(&mut store, "add_ten_i32s")?;

    // let result = add_ten_i32s.call(&mut store, (10, 20, 30, 40))?;
    // assert_eq!(result, (20, 30, 40, 50));

    Ok(())
}
