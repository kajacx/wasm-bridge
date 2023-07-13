# My first component

Here is a simple example on how to define a WIT world, implement it in a Rust plugin,
and call it from Rust runtime on desktop and on the web using the same source code.

## Prerequisites

Be sure to install all the necessary tooling, list is in [Component model](../component_model.md).

## Create the WIT protocol and plugin
  
1. Create a simple file describing the interface using the [wit format](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md), for example:
```wit
package usage:example

world calculator {
  export add: func(a: s32, b: s32) -> s32
}
```

2. Create a new Rust crate for the plugin and add `wit-bindgen` crate as a dependency. Example `Cargo.toml`:
```toml
[package]
name = "plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wit-bindgen = "0.8.0"
```

3. Import the WIT world definition in the plugin like this:
```rust
wit_bindgen::generate!({
    path: "../protocol.wit", // Path to the wit file created earlier
    world: "calculator",
});
```

4. Implement the generated `Calculator` (name based on world name) trait for a custom struct:
```rust
struct MyCalculator;

impl Calculator for MyCalculator {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}
```

5. Export your struct with the `export_calculator` (name based on world name again) macro:
```rust
export_calculator!(MyCalculator);
```

## Build the plugin

1. Build the plugin with `cargo build --target=wasm32-unknown-unknown`

2. `cd` into the build folder `target/wasm32-unknown-unknown/debug` (use `/release` in release mode)

3. Convert the WASM module into a WASM component with `wasm-tools component new plugin.wasm -o component.wasm`

This is the `component.wasm` file you would use with wasmtime normally. We will need it in the next two steps.

4. Convert the WASM component so that it can run on the web with `jco transpile component.wasm --instantiation -o out-dir`

At this point, you can run the component on the web from JS, but that's not what we want.

5. Convert the "web component" _again_ with `wasm-bridge-cli out-dir -o component.zip --universal component.wasm`

Reinstall with `cargo install wasm-bridge-cli -f` to use version at least `0.1.5`.

This prepares the web component so that it can be loaded from Rust on the web.

In universal mode, it also includes the original wasm component, so that the zip file can be used
both on the web and on desktop with `wasm_bridge::component::new_universal_component`.


## Create the runtime

1. Add the `wasm-bridge` crate as a dependency with the `component-model` feature. Use version at least `0.1.4`. Example:
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

Use the `component.zip` file created with `wasm-bridge-cli` and "feed" it into the `new_universal_component` function:

```rust
let component_bytes: &[u8] = /* ... */;

let mut config = Config::new();
config.wasm_component_model(true);

let engine = Engine::new(&config)?;
let mut store = Store::new(&engine, ());

let component = wasm_bridge::component::new_universal_component(&store.engine(), &component_bytes)?;
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

The steps are mostly identical to using wasmtime with component model "normally", but we need to create the universal component
that can run on desktop as well as on the web, and load it with a custom function `new_universal_component` instead of using `Component::new`.

## Next steps

The universal component is great for compatibility, but not for size. If you want to make your component files smaller, check out the [No universal](./no_universal.md) guide.

If your wit world defines imports, you can read the [Wit imports](./wit_imports.md) guide. The code is identical to wasmtime, though.
