use std::error::Error;
use wasm_bridge::*;

pub fn run_test(bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut store = Store::<()>::default();

    let module = Module::new(&store.engine(), bytes)?;

    let instance = Instance::new(&mut store, &module, &[])?;

    let add_five_i32 = instance.get_typed_func::<i32, i32>(&mut store, "add_five_i32")?;

    for number in [-10, -1, 0, 10, i32::MIN + 1, i32::MAX - 2] {
        let returned = add_five_i32.call(&mut store, number)?;
        assert_eq!(returned, number.wrapping_add(5));
    }

    let add_five_i64 = instance.get_typed_func::<i64, i64>(&mut store, "add_five_i64")?;

    for number in [-10, -1, 0, 10, i64::MIN + 1, i64::MAX - 2] {
        let returned = add_five_i64.call(&mut store, number)?;
        assert_eq!(returned, number.wrapping_add(5));
    }

    let add_five_u32 = instance.get_typed_func::<u32, u32>(&mut store, "add_five_i32")?;

    for number in [0, 10, u32::MAX / 2 - 1, u32::MAX - 2] {
        let returned = add_five_u32.call(&mut store, number)?;
        assert_eq!(returned, number.wrapping_add(5));
    }

    let add_five_f32 = instance.get_typed_func::<f32, f32>(&mut store, "add_five_f32")?;

    for number in [0.0, 10.25, -2.5, 1_000_000.5, -1_000_000.5] {
        let returned = add_five_f32.call(&mut store, number)?;
        assert_eq!(returned, number + 5.0);
    }

    let add_five_f64 = instance.get_typed_func::<f64, f64>(&mut store, "add_five_f64")?;

    for number in [0.0, 10.25, -2.5, 10_000_000_000.5, -10_000_000_000.5] {
        let returned = add_five_f64.call(&mut store, number)?;
        assert_eq!(returned, number + 5.0);
    }

    Ok(())
}
