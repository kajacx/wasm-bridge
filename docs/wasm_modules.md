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

## Sync vs async methods

When running on the web, compiling and instantiation WASM modules syncly can result in an error.

Wasm-bridge provides async variants, which are safer to use and don't block the JS "thread" the code runs on.

These are, however, slightly different from the "normal" async functions that wasmtime provides, as described in this table:

| Function | signature | feature | Desktop runtime | Web runtime |
| --- | --- | --- | --- | --- |
| `Module::new` | sync | *none* | Calls wasmtime's `Module::new` | Calls the sync `new WebAssembly.Module()` constructor ❌ |
| `wasm_bridge::new_module_async` | async | *none* | Calls wasmtime's `Module::new` | Calls the async `WebAssembly.compile()` function ✅ |
| --- | --- | --- | --- | --- |
| `Instance::new` | sync | *none* | Calls wasmtime's `Instance::new` | Calls the sync `new WebAssembly.Instance()` constructor ❌ |
| `Instance::new_async` | async | `async` | Calls wasmtime's `Instance::new_async` | Calls the async `new WebAssembly.instantiate()` function ✅ |
| `wasm_bridge::new_instance_async` | async | *none* | Calls wasmtime's `Instance::new` | Calls the async `new WebAssembly.instantiate()` function ✅ |
| --- | --- | --- | --- | --- |
| `linker.instantiate` | sync | *none* | Calls wasmtime's `linker.instantiate` | Calls the sync `new WebAssembly.Instance()` constructor ❌ |
| `linker.instantiate_async` | async | `async` | Calls wasmtime's `linker.instantiate_async` | Calls the async `new WebAssembly.instantiate()` function ✅ |
| `wasm_bridge::instantiate_async` | async | *none* | Calls wasmtime's `linker.instantiate` | Calls the async `new WebAssembly.instantiate()` function ✅ |
| --- | --- | --- | --- | --- |
| `Component::new` | sync | *none* | Calls wasmtime's `Component::new` | Calls the sync `new WebAssembly.Module()` constructor ❌ |
| `wasm_bridge::component::new_component_async` | async | *none* | Calls wasmtime's `Component::new` | Calls the async `WebAssembly.compile()` function ✅ |

The advantage of the "custom" wasm-bridge methods is that they work even without the `async` feature flag.

The component functions require the `component-model` feature, but that would not fit into the table.
