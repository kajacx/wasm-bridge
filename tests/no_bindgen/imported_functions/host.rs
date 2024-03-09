use std::sync::{Arc, Mutex};
use wasm_bridge::*;

pub async fn run_test(bytes: &[u8]) -> Result<()> {
    super::disable_sync_wasm_functions();

    let mut store = Store::<()>::default();

    // Try new_module_async with module bytes
    let module = new_module_async(store.engine(), bytes).await?;

    let mut linker = Linker::new(store.engine());

    // Single value
    linker.func_wrap("imported_fns", "add_one_i32", |_: Caller<()>, val: i32| {
        (val.wrapping_add(1),) // Test single-value tuple
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

    // Multiple values
    linker.func_wrap(
        "imported_fns",
        "add_i32_import",
        |_: Caller<()>, a: i32, b: i32| a.wrapping_add(b),
    )?;

    // No arguments - use interior mutability, must be Send + Sync + 'static
    let global_value = Arc::new(Mutex::new(5i32));
    let global_clone = global_value.clone();
    linker.func_wrap("imported_fns", "increment", move |_: Caller<()>| {
        let mut lock = global_clone.lock().unwrap();
        *lock = *lock + 1;
    })?;

    // Test "async" instantiate
    let instance = instantiate_async(&mut store, &linker, &module).await?;

    single_value(&mut store, &instance)?;
    few_values(&mut store, instance, global_value)?;
    many_values(&mut store).await?;
    errors(&mut store).await?;

    Ok(())
}

fn single_value(mut store: &mut Store<()>, instance: &Instance) -> Result<()> {
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

fn few_values(
    mut store: &mut Store<()>,
    instance: Instance,
    global_value: Arc<Mutex<i32>>,
) -> Result<()> {
    // Two arguments
    let add_i32 = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add_i32")?;
    let returned = add_i32.call(&mut store, (5, 15))?;
    assert_eq!(returned, 20);

    // No arguments
    let increment_twice = instance.get_typed_func::<(), ()>(&mut store, "increment_twice")?;

    #[allow(dropping_copy_types)]
    drop(instance); // test that exported fns can "live on their own"

    increment_twice.call(&mut store, ())?;
    assert_eq!(*global_value.lock().unwrap(), 7); // Initialized to 5 originally

    Ok(())
}

async fn many_values(mut store: &mut Store<()>) -> Result<()> {
    let wat = r#"(module
        (type $t0 (func (param i32 i64 i32 i64 f32 f64) (result i32 i64 i32 i64 f32 f64)))
        (import "imported_fns" "add_import" (func $add_import (type $t0)))
        (func $add (export "add") (type $t0)
          (param $p0 i32) (param $p1 i64) (param $p2 i32) (param $p3 i64) (param $p4 f32) (param $p5 f64)
          (result i32 i64 i32 i64 f32 f64)
            (call $add_import
              (local.get $p0)
              (local.get $p1)
              (local.get $p2)
              (local.get $p3)
              (local.get $p4)
              (local.get $p5)
            )
        )
        (table $T0 1 1 funcref)
    )
    "#;

    // Try new_module_async with module wat

    let module = new_module_async(store.engine(), wat.as_bytes()).await?;
 
    let mut linker = Linker::new(store.engine());
    linker.func_wrap(
        "imported_fns",
        "add_import",
        |_: Caller<()>, a: i32, b: i64, c: u32, d: u64, e: f32, f: f64| {
            (a + 1, b + 1, c + 1, d + 1, e + 1.0, f + 1.0)
        },
    )?;
    let instance = instantiate_async(&mut store, &linker, &module).await?;
 
    let add = instance
        .get_typed_func::<(i32, i64, u32, u64, f32, f64), (i32, i64, u32, u64, f32, f64)>(
            &mut store, "add",
        )?;
    let returned = add.call(&mut store, (5, 15, 25, 35, 45.5, 55.5))?;
    assert_eq!(returned, (6, 16, 26, 36, 46.5, 56.5));
 
    Ok(())
}

async fn errors(mut store: &mut Store<()>) -> Result<()> {
    let wat = r#"(module
        (type $t0 (func))
        (import "imported_fns" "panics_import" (func $panics_import (type $t0)))
        (func $panics (export "panics") (type $t0)
            (call $panics_import)
        )
    )"#;

    let module = new_module_async(store.engine(), wat.as_bytes()).await?;

    new_instance_async(&mut store, &module, &[]).await
        .map(|_| ())
        .expect_err("no imported functions");

    let mut linker = Linker::new(store.engine());
    linker.func_wrap("wrong_module", "panics_import", |_: Caller<()>| {})?;
    instantiate_async(&mut store, &linker, &module).await
        .map(|_| ())
        .expect_err("wrong module name");

    let mut linker = Linker::new(store.engine());
    linker.func_wrap("imported_fns", "wrong_fn", |_: Caller<()>| {})?;
    instantiate_async(&mut store, &linker, &module).await
        .map(|_| ())
        .expect_err("wrong function name");

    // TODO: these checks don't work. Again, maybe check how wasmer does it?

    // let mut linker = Linker::new(store.engine());
    // linker.func_wrap("imported_fns", "panics_import", |_: Caller<()>, _: i32| {})?;
    // linker
    //     .instantiate(&mut store, &module)
    //     .map(|_| ())
    //     .expect_err("wrong arguments");

    // let mut linker = Linker::new(store.engine());
    // linker.func_wrap("imported_fns", "panics_import", |_: Caller<()>| 5i32)?;
    // linker
    //     .instantiate(&mut store, &module)
    //     .map(|_| ())
    //     .expect_err("wrong results");

    // Panic in imported fn causes panic in caller, we are not going to check that

    Ok(())
}
