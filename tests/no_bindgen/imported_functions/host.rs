use std::error::Error;
use wasm_bridge::*;

pub fn run_test(bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut store = Store::<()>::default();

    let module = Module::new(&store.engine(), bytes)?;

    let mut linker = Linker::new(store.engine());
    linker.func_wrap("imported_fns", "add_one_i32", |_: Caller<()>, val: i32| {
        val.wrapping_add(1)
    })?;
    let instance = linker.instantiate(&mut store, &module)?;

    let add_three_i32 = instance.get_typed_func::<i32, i32>(&mut store, "add_three_i32")?;
    let returned = add_three_i32.call(&mut store, 5)?;
    assert_eq!(returned, 5 + 3);

    Ok(())
}
