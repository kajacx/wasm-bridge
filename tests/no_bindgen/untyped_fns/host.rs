use wasm_bridge::*;

pub async fn run_test(bytes: &[u8]) -> Result<()> {
    single_value(bytes)?;
    multiple_values()?;

    Ok(())
}

fn single_value(bytes: &[u8]) -> Result<()> {
    let mut store = Store::<()>::default();
    let module = Module::new(store.engine(), bytes)?;

    let mut linker = Linker::new(store.engine());
    linker.func_new(
        "imported_fns",
        "add_one_i32",
        FuncType::new([ValType::I32], [ValType::I32]),
        |_: Caller<()>, args: &[Val], rets: &mut [Val]| {
            // TODO: cannot use this match, it will be Val::F64
            // Ok(match args[0] {
            //     Val::I32(val) => rets[0] = Val::I32(*val + 1),
            //     _ => unreachable!(),
            // })
            rets[0] = Val::I32(args[0].i32().unwrap() + 1);
            Ok(())
        },
    )?;
    linker.func_new(
        "imported_fns",
        "add_one_i64",
        FuncType::new([ValType::I64], [ValType::I64]),
        |_: Caller<()>, args: &[Val], rets: &mut [Val]| {
            rets[0] = Val::I64(args[0].i64().unwrap() + 1);
            Ok(())
        },
    )?;
    linker.func_new(
        "imported_fns",
        "add_one_f32",
        FuncType::new([ValType::F32], [ValType::F32]),
        |_: Caller<()>, args: &[Val], rets: &mut [Val]| {
            // TODO: cannot use this match, it will be Val::F64
            // Ok(match args[0] {
            //     Val::F32(val) => rets[0] = Val::F32(f32::from_bits(*val) + 1),
            //     _ => unreachable!(),
            // })
            rets[0] = (args[0].f32().unwrap() + 1.0).into();
            Ok(())
        },
    )?;
    linker.func_new(
        "imported_fns",
        "add_one_f64",
        FuncType::new([ValType::F64], [ValType::F64]),
        |_: Caller<()>, args: &[Val], rets: &mut [Val]| {
            rets[0] = (args[0].f64().unwrap() + 1.0).into();
            Ok(())
        },
    )?;

    let instance = linker.instantiate(&mut store, &module)?;
    let mut results = [Val::I32(0)];

    let add_three_i32 = instance.get_func(&mut store, "add_three_i32").unwrap();
    add_three_i32.call(&mut store, &[Val::I32(5)], &mut results)?;
    assert_eq!(results[0].i32().unwrap(), 8);

    let add_three_i64 = instance.get_func(&mut store, "add_three_i64").unwrap();
    add_three_i64.call(&mut store, &[Val::I64(5)], &mut results)?;
    assert_eq!(results[0].i64().unwrap(), 8);

    let add_three_f32 = instance.get_func(&mut store, "add_three_f32").unwrap();
    add_three_f32.call(&mut store, &[(5.5f32).into()], &mut results)?;
    assert_eq!(results[0].f32().unwrap(), 8.5);

    let add_three_f64 = instance.get_func(&mut store, "add_three_f64").unwrap();
    add_three_f64.call(&mut store, &[(5.5f64).into()], &mut results)?;
    assert_eq!(results[0].f64().unwrap(), 8.5);

    Ok(())
}

fn multiple_values() -> Result<()> {
    let wat = r#"(module
        (type $t0 (func (param i32 i64 f32 f64) (result f64)))
        (type $t1 (func (param) (result)))
        (import "imported_fns" "add_import" (func $add_import (type $t0)))
        (import "imported_fns" "increment" (func $increment (type $t1)))
        (func $add (export "add") (type $t0)
          (param $p0 i32) (param $p1 i64) (param $p2 f32) (param $p3 f64)
          (result f64)
            (call $add_import
              (local.get $p0)
              (local.get $p1)
              (local.get $p2)
              (local.get $p3)
            )
        )
        (func $increment_twice (export "increment_twice") (type $t1)
            (call $increment)
            (call $increment)
        )
        (table $T0 1 1 funcref)
    )"#;

    let mut store = Store::<u32>::default();
    let module = Module::new(store.engine(), wat.as_bytes())?;

    let mut linker = Linker::new(store.engine());
    linker.func_new(
        "imported_fns",
        "add_import",
        FuncType::new(
            [ValType::I32, ValType::I64, ValType::F32, ValType::F64],
            [ValType::F64],
        ),
        |_: Caller<u32>, args: &[Val], rets: &mut [Val]| {
            let result = args[0].i32().unwrap() as f64
                + args[1].i64().unwrap() as f64
                + args[2].f32().unwrap() as f64
                + args[3].f64().unwrap();

            rets[0] = result.into();
            Ok(())
        },
    )?;
    linker.func_new(
        "imported_fns",
        "increment",
        FuncType::new([], []),
        |mut caller: Caller<u32>, _args: &[Val], _rets: &mut [Val]| {
            let value = *caller.data();
            let value = value + 1;
            *caller.data_mut() = value;
            Ok(())
        },
    )?;

    let instance = linker.instantiate(&mut store, &module)?;
    let mut results = [Val::I32(0)];

    let add = instance.get_func(&mut store, "add").unwrap();
    add.call(
        &mut store,
        &[
            Val::I32(5),
            Val::I64(15),
            Val::F32((25.5f32).to_bits()),
            Val::F64((35.25f64).to_bits()),
        ],
        &mut results,
    )?;
    assert_eq!(results[0].f64().unwrap(), 5.0 + 15.0 + 25.5 + 35.25);

    let increment_twice = instance.get_func(&mut store, "increment_twice").unwrap();
    increment_twice.call(&mut store, &[], &mut [])?;
    assert_eq!(*store.data(), 2);

    Ok(())
}
