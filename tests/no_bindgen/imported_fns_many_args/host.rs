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

    Ok(())
}
