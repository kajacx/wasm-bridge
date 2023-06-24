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
    many_args()?;

    Ok(())
}

fn many_args() -> Result<(), Box<dyn Error>> {
    let mut store = Store::<()>::default();

    let wat = r#"(module
      (func $add_ten_all (export "add_ten_all")
        (param $p0 i32) (param $p1 i64) (param $p2 i32) (param $p3 i64) (param $p4 f32) (param $p5 f64) (result i32 i64 i32 i64 f32 f64)
        (i32.add (local.get $p0) (i32.const 10))
        (i64.add (local.get $p1) (i64.const 10))
        (i32.add (local.get $p2) (i32.const 10))
        (i64.add (local.get $p3) (i64.const 10))
        (f32.add (local.get $p4) (f32.const 10))
        (f64.add (local.get $p5) (f64.const 10))
      )
    )"#;

    let module = Module::new(&store.engine(), wat.as_bytes())?;

    let instance = Instance::new(&mut store, &module, &[])?;

    let add_ten_all = instance
        .get_typed_func::<(i32, i64, u32, u64, f32, f64), (i32, i64, u32, u64, f32, f64)>(
            &mut store,
            "add_ten_all",
        )?;
    let returned = add_ten_all.call(&mut store, (10, 20, 30, 40, 50.25, 60.5))?;
    assert_eq!(returned, (20, 30, 40, 50, 60.25, 70.5));

    Ok(())
}
