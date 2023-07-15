# Component model

Since version `0.1.2`, it is now possible to use the [wit component model](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md) with the `component-model` feature.

## Pre-requirements

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Add WASM target with `rustup target add wasm32-unknown-unknown`
3. [Install `wasm-pack`](https://rustwasm.github.io/wasm-pack/installer) with `cargo install wasm-pack`
4. [Install `wasm-tools`](https://github.com/bytecodealliance/wasm-tools) with `cargo install wasm-tools`
5. [Install node and npm](https://nodejs.org/en/download)
6. [Install `jco`](https://github.com/bytecodealliance/jco) with `npm install -g @bytecodealliance/jco` (use version at least `0.9.4`)
7. Install `wasm-bridge-cli` with `cargo install wasm-bridge-cli` (use `-f` to install the newest version)

Steps 1-4 are the same when using wasmtime's component model

Steps 1-6 are the same when running a wit component on the web from JS "the intended way".

## Project setup

Full (minimal viable) example project setup can be found [here](https://github.com/kajacx/wasm-playground/tree/wasm-bridge-03-universal-component).

Alternatively, the [My first component](CM/my_first_component.md) doc
contains detailed step-by-step tutorial on how to compile, load and run a wasm component.

If your world has imports, you can read [WIT imports](CM/wit_imports.md) on how to define and use them.

If you want to optimize for size, you can also check out
the [No universal](CM/no_universal.md) guide.

## Implemented features

- All primitive types (numbers, char, bool, string) supported
- Exported and imported functions with 0-N arguments and 0-N return values
- `list`, `option` and `tuple` types
- Custom `record` and `variant` types
- Use the same component file on sys and web with the "universal component"

See the [`wit_components`](/tests/wit_components) test folder for supported example usages.
