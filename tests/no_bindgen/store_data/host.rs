use wasm_bridge::*;

struct Data {
    times_called: u32,
}

pub async fn run_test(bytes: &[u8]) -> Result<()> {
    assert_send::<Store<()>>();
    assert_sync::<Store<()>>();

    let data = Data { times_called: 0 };

    let engine = Engine::default();
    let mut store = Store::new(&engine, data);

    #[allow(deprecated)]
    let module = Module::new(store.engine(), bytes).unwrap();

    let mut linker = Linker::<Data>::new(store.engine());

    // Single value
    linker.func_wrap(
        "imported_fns",
        "add_one_i32",
        |caller: Caller<Data>, val: i32| {
            increment(caller);
            val.wrapping_add(1)
        },
    ).unwrap();

    #[allow(deprecated)]
    let instance = linker.instantiate(&mut store, &module).unwrap();

    let add_three_i32 = instance.get_typed_func::<i32, i32>(&mut store, "add_three_i32").unwrap();

    add_three_i32.call(&mut store, 5).unwrap();
    add_three_i32.call(&mut store, 10).unwrap();

    assert_eq!(store.data().times_called, 2);

    // Re-use the linker
    #[allow(deprecated)]
    let instance = linker.instantiate(&mut store, &module).unwrap();

    let add_three_i32 = instance.get_typed_func::<i32, i32>(&mut store, "add_three_i32").unwrap();

    add_three_i32.call(&mut store, 5).unwrap();
    add_three_i32.call(&mut store, 10).unwrap();

    assert_eq!(store.data().times_called, 4);

    // Test memory api
    let memory = instance.get_memory(&mut store, "memory").unwrap();
    let mut bytes: [u8; 3] = [5, 6, 7];

    let address = instance
        .get_typed_func::<u32, u32>(&mut store, "allocate_bytes")?
        .call(&mut store, bytes.len() as _).unwrap();

    memory.write(&mut store, address as usize, &bytes).unwrap();
    instance
        .get_typed_func::<(u32, u32), ()>(&mut store, "increment_bytes_at")?
        .call(&mut store, (address, bytes.len() as _)).unwrap();
    memory.read(&mut store, address as usize, &mut bytes).unwrap();

    assert_eq!(bytes, [6, 7, 8]);

    Ok(())
}

// mut in unneeded on sys, since data is a normal reference there
#[allow(unused_mut)]
fn increment(mut context: impl AsContextMut<Data = Data>) {
    let mut store = context.as_context_mut();
    let mut data = store.data_mut();
    data.times_called += 1;
}

fn assert_sync<T: Sync>() {}

fn assert_send<T: Send>() {}
