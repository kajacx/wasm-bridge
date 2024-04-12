# wasm-bridge change log

## [0.3.2] 2024-04-12

### Added

- Made `Store` `Sync` and `Send`.


## [0.3.1] 2024-03-10

### Changes

- Improved performance.
- Fixed module namespace hack, allowing records with similar names.
- Fixed "async fn" replacement hack.

## [0.3.0] 2024-01-07

### Added

- Fixed WASI examples, tested with `cargo component` version `0.5.0`.
- Environment variables support for WASI.

### Changes

- Objects are now sent directly to wasm modules as bytes instead of creating a JS object for the jco-generate code,
making passing objects much more efficient.

### <b style="color: red">Breaking changes:</b>

- Updated to wasmtime 15.0. This changed the custom IO redirection traits.
- Moved the `wasm_bridge::wasi` module to a separate `wasm_bridge_wasi` crate.
- Async user imported functions are not unsupported, use `async: { only_imports: [] }` to make your world imports synchronous.
- Renamed `component_new_async` to `new_component_async`.

## [0.2.2] 2023-08-03

### Added

- `new_component_async` function.
- Wasi default and custom random generators.
- Memory view api.
- Untyped functions with the `Val` type.

### Changes

- Removed lifetime from `TypedFunc`.
- `Caller` now implements `AsContext[Mut]`.

## [0.2.1] 2023-07-24

### Added

- Added first "MVP" wasi support.
- Inherit std/out, redirect std in/out/err in wasi.

### Changed

- Replace `wasm-bridge-jco` with [`js-component-bindgen`](https://crates.io/crates/js-component-bindgen).


## [0.2.0] 2023-07-21

### Changed

- Updated wasmtime to version `11.0.0` on sys.
- `Component::new()` now takes the same component bytes on desktop and on the web.

### <b style="color: red">Breaking changes:</b>

- `wasm-bridge-cli` is removed, as you can load components with `Component::new()` directly.
- Loading "zipped" components created with earlier version of `wasm-bridge-cli` will not work.
- `new_universal_component` is completely removed, as well as the `component-model-no-universal` flag.


## [0.1.6] 2023-07-20

### Added

- Added enum support to wit.
- Added result support to wit.
- Added better error messages and logging.
- Added interface support to wit.

### Changed

- Changed result type to be `anyhow::Result` to match wasmtime.


## [0.1.5] 2023-07-13

### Added

- Added variant support to wit.

### Changed

- Out file in cli is defined with an -o option. Old way works for now.


## [0.1.4] 2023-07-12

### Added

- Universal component zip support.
- Support rest of the primitives in wit bindgen.
- Add custom records support to wit format.


## [0.1.3] 2023-07-11

### Added

- Tuple return in WIT imported and exported fns.
- Support option and list in WIT format.


## [0.1.2] 2023-07-09

### Added

- Wit component support:
- Imported and exported function, 0-8 arguments, 0-1 returns.
- Only primitives and String types supported.
- See [Component model](/component_model.md).
- `Error` is now `Send` and `Sync`.
- Custom store data, accessible from Caller (imported fn).


## [0.1.1] 2023-07-05

### Added

- Export `Result` type.


## [0.1.0] 2023-07-02

### Added

- Load and instantiate a module.
- Get and call typed exported functions.
- Define imported functions with a linker.
- Supported types: `i32`, `i64`, `u32`, `u64`, `f32`, `f64`.
- Multi-value returns from exported and imported functions.
