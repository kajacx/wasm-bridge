# wasm-bridge

## Goals

The goal of this crate is to "run wasmtime on the web".

Since wasmtime cannot actually *run* on the web, the goal is to **provide a unified API** for both sys (desktop) and js (web) runtimes.

## How to use

The provided API is identical to wasmtime's API, so read [wasmtime's documentation](https://docs.wasmtime.dev/) on how to use this crate.

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
    let module = Module::new(&store.engine(), wat.as_bytes())?;

    let instance = Instance::new(&mut store, &module, &[])?;
    let add_three_wasm = instance.get_typed_func::<i32, i32>(&mut store, "add_three")?;

    let result = add_three_wasm.call(&mut store, number)?;
    Ok(result)
}
```

## Switching from `wasmtime`

Simply replace the `wasmtime` dependency and imports with `wasm-bridge`, and you *should* be good to go.

Most of wasmtime's API is still not implemented, so you will likely run into compile errors when compiling to wasm.

Create an issue with a code snippet describing your use case.

## Using `component-model`

Work on supporting the component model has only just begun, but it will hopefully be possible to use the component model with `wasm-bridge` in the future.

## Implemented features
- Load a module from bytes
- Instantiate a module with an empty import object
- Get (typed) exported function and call it
- Add imported functions to a linker
- Instantiate a module with a linker to use imported functions
- Compile a module from the wat format
- Multivalue returns from exported and imported functions
- Supported value types: `i32`, `i64`, `u32`, `u64`, `f32`, `f64`

See the [`no_bindgen`](/tests/no_bindgen) folder for supported example usages.
