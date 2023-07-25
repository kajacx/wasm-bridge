use wasm_bridge::*;

pub fn run_test(bytes: &[u8]) -> Result<()> {
    single_value(bytes)?;

    Ok(())
}

fn single_value(bytes: &[u8]) -> Result<()> {
    let mut store = Store::<()>::default();

    let module = Module::new(store.engine(), bytes)?;

    let mut linker = Linker::new(store.engine());

    // Single value
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
    add_three_f32.call(&mut store, &[(5.0f32).into()], &mut results)?;
    assert_eq!(results[0].f32().unwrap(), 8.0);

    let add_three_f64 = instance.get_func(&mut store, "add_three_f64").unwrap();
    add_three_f64.call(&mut store, &[(5.0f64).into()], &mut results)?;
    assert_eq!(results[0].f64().unwrap(), 8.0);

    Ok(())
}
