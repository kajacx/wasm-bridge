# My first component

Here is a simple example on how to define a WIT world, implement it in Rust guest (plugin),
and call it from Rust runtime on desktop and on the web using the same source code.

## Full minimal example

Full minimal example can be found [here](https://github.com/kajacx/wasm-playground/tree/wasm-bridge-05-cargo-component).

## Prerequisites

Be sure to install all the necessary tooling, list is in [Component model](../component_model.md).

## Create the WIT protocol
  
1. Create a simple file describing the interface using
the [wit format](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md), for example:

```wit
package usage:example

world calculator {
  export add_three: func(a: s32) -> s32
}
```

## Create the Rust guest

1. Create a new Rust library crate for the guest and add `wit-bindgen` crate as a dependency. Example `Cargo.toml`:
```toml
[package]
name = "guest"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[package.metadata.component]
package = "component:guest"

[dependencies]
wit-bindgen = "0.8.0"
```

Be sure to specify `cdylib` as the library type and include the `package.metadata.component` info
so that `cargo component` can recognize the crate as a proper wasm component.

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
    fn add_three(num: i32) -> i32 {
        num + 3
    }
}
```

4. Export your struct with the `export_calculator` (name based on world name again) macro:
```rust
export_calculator!(MyCalculator);
```

## Build the guest

1. Build the guest with `cargo component --target=wasm32-unknown-unknown`

2. This should generate a `guest.wasm` file in the `target/wasm32-unknown-unknown/debug` folder

3. Your wasm component will be in `target/wasm32-unknown-unknown/debug` if you use `--release`


## Create the runtime

1. Create a new Rust crate with `wasm-bridge` as a dependency with the `component-model` feature. Use version at least `0.2.2`. Example:
```toml
[dependencies]
wasm-bridge = { version = "0.2.2", features = ["component-model"] }
```

2. Generate the host bindings with the `bindgen` macro, like this:
```rust
wasm_bridge::component::bindgen!({
    path: "../protocol.wit", // Path to the same wit file created earlier
    world: "calculator",
});
```

3. Prepare the store with component model enabled in the config:
```rust
let mut config = Config::new();
config.wasm_component_model(true);

let engine = Engine::new(&config)?;
let mut store = Store::new(&engine, ());

```

4. Load the component from bytes synchronously (not recommended):
```rust
let component_bytes: &[u8] = /* read the file bytes */;

let component = Component::new(&store.engine(), &component_bytes)?;
```

This can work, but browsers can throw "cannot compile large WASM modules on the main thread".

To avoid this, use the async component new function instead.

4. Load the component from bytes asynchronously:
```rust
use wasm_bridge::component::new_component_async;

let component_bytes: &[u8] = /* read the file bytes */;

let component = new_component_async(&store.engine(), &component_bytes).await?;
```

This adds the "hassle" of managing the asynchronous code, but is more robust and it won't "block"
the JS thread for as long.

The function is added on the desktop target as well, so you can safely using without worrying about target arch.

5. Instantiate the component with a linker, like this:
```rust
let linker = Linker::new(store.engine());
let (calculator, _) = Calculator::instantiate(&mut store, &component, &linker)?;
```

6. Call the exported function on the component instance:
```rust
let result = calculator.call_add_three(&mut store, 5)?;
assert_eq!(result, 8);
```


## Summary

The steps are identical to using wasmtime with component model "normally",
except we used `wasm-bridge` instead of `wasmtime` as our dependency.


## Next steps

If your wit world defines imports, you can read the [Wit imports](./wit_imports.md) guide.

The code is again the same as when using wasmtime.


## Universal mode / zipped components discontinuation

Previously, you had to convert the wasm component with `jco` and `wasm-bridle-cli`.

Since wasm-bridge version 0.2.0, this is *no longer the case*,
and you can use the component bytes in `Component::new()` on desktop as well as on the web.
