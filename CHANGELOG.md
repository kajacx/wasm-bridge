# wasm-bridge change log

## [0.2.0] YYYY-MM-DD

### Planned <b style="color: red">breaking changes</b>:

- Remove out file as the second argument in cli, only the -o option will remain.

## [0.1.6] 2023-07-20

### Added

- Added enum support to wit
- Added result support to wit
- Added better error messages and logging
- Added interface support to wit

### Changed

- Changed result type to be anyhow::Result to match wasmtime

## [0.1.5] 2023-07-13

### Added

- Added variant support to wit

### Changed
- Out file in cli is defined with an -o option. Old way works for now.

## [0.1.4] 2023-07-12

### Added

- Universal component zip support
- Support rest of the primitives in wit bindgen
- Add custom records support to wit format

## [0.1.3] 2023-07-11

### Added

- Tuple return in WIT imported and exported fns
- Support option and list in WIT format

## [0.1.2] 2023-07-09

### Added

- Wit component support:
- Imported and exported function, 0-8 arguments, 0-1 returns
- Only primitives and String types supported
- See [Component model](/component_model.md)
- Error is now Send and Sync
- Custom store data, accessible from Caller (imported fn)

## [0.1.1] 2023-07-05

### Added

- Export `Result` type

## [0.1.0] 2023-07-02

### Added

- Load and instantiate a module
- Get and call typed exported functions
- Define imported functions with a linker
- Supported types: `i32`, `i64`, `u32`, `u64`, `f32`, `f64`
- Multi-value returns from exported and imported functions
