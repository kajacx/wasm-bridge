use std::error::Error;
use wasm_bridge::*;

pub fn run_test(bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut store = Store::<()>::default();

    let module = Module::new(&store.engine(), bytes)?;

    let mut linker = Linker::new(store.engine());
    linker.func_wrap(
        "imported_fns",
        "add_i32_import",
        |_: Caller<()>, a: i32, b: i32| a.wrapping_add(b),
    )?;
    linker.func_wrap(
        "imported_fns",
        "add_sub_ten_import",
        |_: Caller<()>, num: i32| (num.wrapping_add(10), num.wrapping_sub(10)),
    )?;
    let instance = linker.instantiate(&mut store, &module)?;

    // Two arguments
    let add_i32 = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add_i32")?;
    let returned = add_i32.call(&mut store, (5, 15))?;
    assert_eq!(returned, 20);

    // Two return values
    let add_sub_ten = instance.get_typed_func::<i32, (i32, i32)>(&mut store, "add_sub_ten")?;
    let returned = add_sub_ten.call(&mut store, 20)?;
    assert_eq!(returned, (30, 10));

    // few_args()?;
    many_args()?;

    Ok(())
}

fn few_args() -> Result<(), Box<dyn Error>> {
    let mut store = Store::<()>::default();

    let wat = r#"(module
        (type $t0 (func (param i32 i32 i32) (result i32 i32 i32)))
        (import "imported_fns" "add_import" (func $add_import (type $t0)))
        (func $add (export "add") (type $t0)
          (param $p0 i32) (param $p1 i32) (param $p2 i32)
          (result i32 i32 i32)
            (call $add_import
              (local.get $p0)
              (local.get $p1)
              (local.get $p2)
            )
        )
        (table $T0 1 1 funcref)
    )
    "#;

    let module = Module::new(&store.engine(), wat.as_bytes())?;

    let mut linker = Linker::new(store.engine());
    linker.func_wrap(
        "imported_fns",
        "add_import",
        |_: Caller<()>, a: i32, b: i64, c: u32, d: u64, e: f32, f: f64| {
            (a + 1, b + 1, c + 1, d + 1, e + 1.0, f + 1.25)
        },
    )?;
    let instance = linker.instantiate(&mut store, &module)?;

    let add = instance
        .get_typed_func::<(i32, i64, u32, u64, f32, f64), (i32, i64, u32, u64, f32, f64)>(
            &mut store, "add",
        )?;
    let returned = add.call(&mut store, (5, 15, 25, 35, 45.5, 55.5))?;
    assert_eq!(returned, (6, 16, 26, 36, 46.5, 56.5));

    Ok(())
}

fn many_args() -> Result<(), Box<dyn Error>> {
    let mut store = Store::<()>::default();

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

    let module = Module::new(&store.engine(), wat.as_bytes())?;

    let mut linker = Linker::new(store.engine());
    linker.func_wrap(
        "imported_fns",
        "add_import",
        |_: Caller<()>, a: i32, b: i64, c: u32, d: u64, e: f32, f: f64| {
            (a + 1, b + 1, c + 1, d + 1, e + 1.0, f + 1.0)
        },
    )?;
    let instance = linker.instantiate(&mut store, &module)?;

    let add = instance
        .get_typed_func::<(i32, i64, u32, u64, f32, f64), (i32, i64, u32, u64, f32, f64)>(
            &mut store, "add",
        )?;
    let returned = add.call(&mut store, (5, 15, 25, 35, 45.5, 55.5))?;
    assert_eq!(returned, (6, 16, 26, 36, 46.5, 56.5));

    Ok(())
}
