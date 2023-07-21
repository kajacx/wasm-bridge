# My first component

Here is a simple example on how to define a WIT world, implement it in Rust guest (plugin),
and call it from Rust runtime on desktop and on the web using the same source code.

## Full minimal example

Full minimal example can be found [here](https://github.com/kajacx/wasm-playground/tree/wasm-bridge-03-universal-component).

## Prerequisites

Be sure to install all the necessary tooling, list is in [Component model](../component_model.md).

## Create the WIT protocol
  
1. Create a simple file describing the interface using the [wit format](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md), for example:
```wit
package usage:example

world calculator {
  export add: func(a: s32, b: s32) -> s32
}
```

## Create the Rust guest

1. Create a new Rust crate for the guest and add `wit-bindgen` crate as a dependency. Example `Cargo.toml`:
```toml
[package]
name = "guest"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wit-bindgen = "0.8.0"
```

2. Import the WIT world definition in the guest like this:
```rust
wit_bindgen::generate!({
    path: "../protocol.wit", // Path to the wit file created earlier
    world: "calculator",
});
```

3. Implement the generated `Calculator` (name based on world name) trait for a custom struct:
```rust
struct MyCalculator;

impl Calculator for MyCalculator {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}
```

4. Export your struct with the `export_calculator` (name based on world name again) macro:
```rust
export_calculator!(MyCalculator);
```

## Build the guest

1. Build the guest with `cargo build --target=wasm32-unknown-unknown`

2. `cd` into the build folder `target/wasm32-unknown-unknown/debug` (use `/release` in release mode)

3. Convert the WASM module into a WASM component with `wasm-tools component new guest.wasm -o component.wasm`

This is the `component.wasm` file you would use with wasmtime normally. We will need it later.

## Create the runtime

1. Add the `wasm-bridge` crate as a dependency with the `component-model` feature. Use version at least `0.2.0`. Example:
```toml
[dependencies]
wasm-bridge = { version = "0.1.4", features = ["component-model"] }
```

2. Generate the host bindings with the `bindgen` macro, like this:
```rust
wasm_bridge::component::bindgen!({
    path: "../protocol.wit", // Path to the same wit file created earlier
    world: "calculator",
});
```

3. Create a `Component` from the component bytes.

```rust
let component_bytes: &[u8] = /* read the file bytes */;

let mut config = Config::new();
config.wasm_component_model(true);

let engine = Engine::new(&config)?;
let mut store = Store::new(&engine, ());

let component = Component::new(&store.engine(), &component_bytes)?;
```

4. Instantiate the component with a linker, like this:
```rust
let linker = Linker::new(store.engine());
let (calculator, _) = Calculator::instantiate(&mut store, &component, &linker)?;
```

5. Call the exported function on the component:
```rust
let result = calculator.call_add(&mut store, 5, 3)?;
assert_eq!(result, 8);
```

## Summary

The steps are identical to using wasmtime with component model "normally",
except we used `wasm-bridge` instead of `wasmtime` as our dependency.

## Next steps

If your wit world defines imports, you can read the [Wit imports](./wit_imports.md) guide. The code is identical to wasmtime, though.

## Universal mode / zipped components discontinuation

Previously, you had to convert the wasm component with `jco` and `wasm-bridle-cli`.

Since wasm-bridge version 0.2.0, this is *no longer the case*,
and you can use the component bytes in `Component::new()` on desktop as well as on the web.
