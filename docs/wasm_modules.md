# Non-component model use

You can use wasm-bridge without the component model, but not many features are implemented.


## Implemented features

- Load a module from bytes or from WAT text
- Instantiate a module with or without imports
- Get (typed) exported function and call it
- Multivalue returns from exported and imported functions
- Supported value types: `i32`, `i64`, `u32`, `u64`, `f32`, `f64`
- Access store's data from Caller (imported fn)

See the [`no_bindgen`](/tests/no_bindgen) test folder for supported example usages.


## Example usage

Here is an example of adding three using a WASM module compiled from WAT, using `wasm-bridge` version `0.1.1`:

```rust
use wasm_bride::*;

fn add_three(number: i32) -> Result<i32> {
    let mut store = Store::<()>::default();

    let wat = r#"
      (module
        (func $add_three (export "add_three")
          (param $p0 i32) (result i32)
          (i32.add (local.get $p0) (i32.const 3))
        )
      )
    "#;
    let module = Module::new(store.engine(), wat.as_bytes())?;

    let instance = Instance::new(&mut store, &module, &[])?;
    let add_three_wasm = instance.get_typed_func::<i32, i32>(&mut store, "add_three")?;

    let result = add_three_wasm.call(&mut store, number)?;
    Ok(result)
}
```

Alternatively, watch the [video tutorial](https://youtu.be/CqpZjouAOvg):

[![Youtube video](https://img.youtube.com/vi/CqpZjouAOvg/0.jpg)](https://youtu.be/CqpZjouAOvg)
