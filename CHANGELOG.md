# wasm-bridge change log

## [0.1.2] YYYY-MM-DD

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
