# wasm-bridge-jco

This is a helper crate for `wasm-bridge`, see [crates.io](https://crates.io/crates/wasm-bridge)
or the [GitHub repo](https://github.com/kajacx/wasm-bridge#wasm-bridge).

### Source

This crate is a fork of the [`js-component-bindgen`](https://github.com/bytecodealliance/jco/tree/main/crates/js-component-bindgen)
crate 

### License

The code in this folder is licensed under the "Apache-2.0 WITH LLVM-exception" license,
see [LICENSE](./LICENSE).

### Changes

- Changed name to `wasm-bridge-jco`
- Changed Cargo.toml metadata (categories, etc.) to wasm bridge workspace
- Changed dependencies to use specific versions instead of workspace versions
- Changed `transpile` to take a byte slice (see [#113](https://github.com/bytecodealliance/jco/pull/113))
- Disabled unused warnings and clippy warnings
