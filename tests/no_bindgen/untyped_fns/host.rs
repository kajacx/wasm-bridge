use wasm_bridge::*;

pub fn run_test(_bytes: &[u8]) -> Result<()> {
    single_value()?;

    Ok(())
}

fn single_value() -> Result<()> {
    let mut store = Store::<()>::default();

    let wat = r#"
    (module
        (type $t0 (func (param i32) (result i32)))
        (import "imported_fns" "add_one_i32" (func $add_one_i32 (type $t0)))
        (func $add (export "add_three_i32") (type $t0)
          (param $p0 i32)
          (result i32)
            (call $add_one_i32
                (call $add_one_i32
                    (call $add_one_i32(local.get $p0))
                )
            )
        )
        (table $T0 1 1 funcref)
    )
    "#;
    let module = Module::new(store.engine(), wat.as_bytes())?;

    let mut linker = Linker::new(store.engine());

    // Single value
    linker.func_new(
        "imported_fns",
        "add_one_i32",
        FuncType::new([ValType::I32], [ValType::I32]),
        |_: Caller<()>, args: &[Val], rets: &mut [Val]| {
            // TODO: cannot use this match, it will be Val::F64
            // Ok(match args[0] {
            //     Val::I32(val) => rets[0] = Val::I32(val + 1),
            //     _ => unreachable!(),
            // })
            rets[0] = Val::I32(args[0].i32().unwrap() + 1);
            Ok(())
        },
    )?;
    let instance = linker.instantiate(&mut store, &module)?;

    let add_three_i32 = instance.get_func(&mut store, "add_three_i32").unwrap();

    let mut results = [Val::I32(0)];
    add_three_i32.call(&mut store, &[Val::I32(5)], &mut results)?;
    assert_eq!(results[0].i32().unwrap(), 8);

    Ok(())
}
