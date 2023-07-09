# wasm-bridge

<div align="center">
  <p>
    <img src="/wasm-bridge-banner.png" alt="image banner" width="600" />
  </p>

  <p>
    <a href="https://www.flaticon.com/" title="Icons from flaticon.com">
      <img src="https://img.shields.io/badge/Icons_from-Flaticon-green" alt="Icons from flaticon.com">
    </a>
    <a href="https://crates.io/crates/wasm-bridge" title="View on crates.io">
      <img src="https://img.shields.io/badge/View_on-crates.io-blue" alt="View on crates.io">
    </a>
    <a href="https://discord.gg/7fk5Uk6Eqr" title="Join the Discord server">
      <img src="https://img.shields.io/discord/1125842158914646080?logo=discord&label=Join" alt="Join the Discord server">
    </a>
  </p>
</div>

## Goals

The goal of this crate is to "run wasmtime on the web".

Since wasmtime cannot actually _run_ on the web, the goal is to **provide a unified API** for both sys (desktop) and js (web) runtimes.

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
    let module = Module::new(store.engine(), wat.as_bytes())?;

    let instance = Instance::new(&mut store, &module, &[])?;
    let add_three_wasm = instance.get_typed_func::<i32, i32>(&mut store, "add_three")?;

    let result = add_three_wasm.call(&mut store, number)?;
    Ok(result)
}
```

## Switching from `wasmtime`

Simply replace the `wasmtime` dependency and imports with `wasm-bridge`, and you _should_ be good to go.

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
