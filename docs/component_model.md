# Component model

It is possible to use the [wit component model](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md) with the `component-model` feature.

## Pre-requirements

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Add WASM target with `rustup target add wasm32-unknown-unknown`
3. [Install `wasm-component`](https://github.com/bytecodealliance/cargo-component)
   with `cargo install --git https://github.com/bytecodealliance/cargo-component cargo-component`


These steps are the same when running a wit component in wasmtime normally.

## Project setup

Full (minimal viable) example project setup can be found [here](https://github.com/kajacx/wasm-playground/tree/wasm-bridge-03-universal-component).

Alternatively, the [My first component](CM/my_first_component.md) doc
contains detailed step-by-step tutorial on how to compile, load and run a wasm component.

If your world has imports, you can read [WIT imports](CM/wit_imports.md) on how to define and use them.

## Implemented features

- All primitive types (numbers, char, bool, string) supported
- Exported and imported functions with 0-N arguments and 0-N return values
- Built-in `list`, `option`, `tuple` and `result` types
- Custom `record`, `enum` and `variant` types
- Imported and exported interfaces

See the [`wit_components`](/tests/wit_components) test folder for supported example usages.
