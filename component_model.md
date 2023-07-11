# Component model

Since version `0.1.2`, it is now possible to use the [wit component model](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md) with the `component-model` feature.

## Pre-requirements

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Add WASM target with `rustup target add wasm32-unknown-unknown`
3. [Install `wasm-pack`](https://rustwasm.github.io/wasm-pack/installer) with `cargo install wasm-pack`
4. [Install `wasm-tools`](https://github.com/bytecodealliance/wasm-tools) with `cargo install wasm-tools`
5. [Install node and npm](https://nodejs.org/en/download)
6. [Install `jco`](https://github.com/bytecodealliance/jco) with `npm install -g @bytecodealliance/jco` (use version at least `0.9.4`)
7. Install `wasm-bridge-cli` with `cargo install wasm-bridge-cli`

Steps 1-4 are the same when using wasmtime's component model

Steps 1-6 are the same when running a wit component on the web from JS "the intended way".

## Project setup

Full (minimal viable) example project setup can be found [here](https://github.com/kajacx/wasm-playground/tree/wasm-bridge-02).

Alternatively, you can follow these steps to get started:

<details>
  <summary>I. Create the WIT protocol and plugin</summary>
  
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
</details>

<details>
  <summary>II. Build the plugin</summary>

  1. Build the plugin with `cargo build --target=wasm32-unknown-unknown`

  2. `cd` into the build folder `target/wasm32-unknown-unknown/debug` (use `/release` in release mode)

  3. Convert the WASM module into a WASM component with `wasm-tools component new plugin.wasm -o component.wasm`

Remember this `component.wasm` file. It's the same one you use in `wasmtime` "normally" and we will need it later.

  4. Convert the WASM so that it can run on the web with `jco transpile component.wasm --instantiation -o out-dir`

At this point, you can run the component on the web from JS, but that's not what we want.

  5. Convert the "web component" _again_ with `wasm-bridge-cli out-dir component.zip`

This _massages_ the web component so that it can be loaded from Rust on the web. We will need this `component.zip` file later.

</details>

<details>
  <summary>III. Create the runtime</summary>

  1. Add the `wasm-bridge` crate as a dependency with the `component-model` feature. Use version at least `0.1.2`. Example:
```toml
[dependencies]
wasm-bridge = { version = "0.1.3", features = ["component-model"] }
```

  2. Generate the host bindings with the `bindgen` macro, like this:
```rust
wasm_bridge::component::bindgen!({
    path: "../protocol.wit", // Path to the same wit file created earlier
    world: "calculator",
});
```

  3. Create a `Component` from the component bytes. __Be sure to use the `component.wasm` file on `sys` (desktop) and the `component.zip` file on `js` (browser)__
```rust
let component_bytes: &[u8] = /* ... */;

let mut config = Config::new();
config.wasm_component_model(true);

let engine = Engine::new(&config)?;
let mut store = Store::new(&engine, ());

let component = Component::new(&store.engine(), &component_bytes)?;
```

  4. Instantiate the component with a linker, like this:
```rust
let linker = Linker::new(store.engine());
let (instance, _) = Calculator::instantiate(&mut store, &component, &linker)?;
```

  5. Call the exported function on the component:
```rust
let result = instance.call_add(&mut store, 5, 3)?;
assert_eq!(result, 8);
```

</details>
<br>

This is mostly identical to using wasmtime's component model "normally", with the added step II.5 of converting the jco web component and the __need to use different component file on the web and on the desktop__.

This need make it slightly awkward to use, but a workaround is planned for a future release where the same component file would work on desktop and on the web.

## Implemented features

- Supported types: 32-bit and 64-bit numbers, string
- Exported and imported functions with 0-N arguments and 0-N return values
- `list<T>` and `option<T>` types

See the [`wit_components`](/tests/wit_components) test folder for supported example usages.
