use wasm_bridge::*;

struct Data {
    times_called: u32,
}

pub fn run_test(bytes: &[u8]) -> Result<()> {
    let data = Data { times_called: 0 };

    let engine = Engine::default();
    let mut store = Store::new(&engine, data);

    let module = Module::new(store.engine(), bytes)?;

    let mut linker = Linker::<Data>::new(store.engine());

    // Single value
    linker.func_wrap(
        "imported_fns",
        "add_one_i32",
        |caller: Caller<Data>, val: i32| {
            increment(caller);
            val.wrapping_add(1)
        },
    )?;

    let instance = linker.instantiate(&mut store, &module)?;

    let add_three_i32 = instance.get_typed_func::<i32, i32>(&mut store, "add_three_i32")?;

    add_three_i32.call(&mut store, 5)?;
    add_three_i32.call(&mut store, 10)?;

    assert_eq!(store.data().times_called, 2);

    // Re-use the linker
    let instance = linker.instantiate(&mut store, &module)?;

    let add_three_i32 = instance.get_typed_func::<i32, i32>(&mut store, "add_three_i32")?;

    add_three_i32.call(&mut store, 5)?;
    add_three_i32.call(&mut store, 10)?;

    assert_eq!(store.data().times_called, 4);

    Ok(())
}

// mut in unneeded on sys, since data is a normal reference there
#[allow(unused_mut)]
fn increment(mut context: impl AsContextMut<Data = Data>) {
    let mut store = context.as_context_mut();
    let mut data = store.data_mut();
    data.times_called += 1;
}
