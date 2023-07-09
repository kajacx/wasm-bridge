# Component model

Since version `0.1.2`, it is now possible to use the [wit component model](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md) with the `component-model` feature.

## Pre-requirements

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Add WASM target with `rustup target add wasm32-unknown-unknown`
3. [Install `wasm-pack`](https://rustwasm.github.io/wasm-pack/installer) with `cargo install wasm-pack`
4. [Install `wasm-tools`](https://github.com/bytecodealliance/wasm-tools) with `cargo install wasm-tools`
5. [Install node and npm](https://nodejs.org/en/download)
6. [Install `jco`](https://github.com/bytecodealliance/jco) with `npm install -g @bytecodealliance/jco`
7. Install `wasm-bridge-cli` with `cargo install wasm-bridge-cli`

Steps 1-4 are the same when using wasmtime's component model

Steps 1-6 are the same when running a wit component on the web from JS "the intended way".

## Project setup

TODO

## Implemented features

- Supported types: 32-bit and 64-bit numbers, string
- Exported and imported functions with 0-N arguments and 0-N return values

See the [`wit_bindgen`](/tests/wit_bindgen) test folder for supported example usages.
