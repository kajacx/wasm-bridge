### Source

This folder is an (almost) direct copy of the
[`component-macro`](https://github.com/bytecodealliance/wasmtime/tree/main/crates/component-macro)
crate in wasmtime.

### License

The code in this folder is licensed under the "Apache-2.0 WITH LLVM-exception" license,
see [LICENSE](./LICENSE).

### Changes

- Renamed `lib.rs` to `mod.rs`
- Removed `#[proc_macro]` and `#[proc_macro_derive(...)]` attributes
- Re-exported `component::Style` and `component::VariantStyle`
- Fixed clippy lint issues, see [#7698](https://github.com/bytecodealliance/wasmtime/pull/7698)
