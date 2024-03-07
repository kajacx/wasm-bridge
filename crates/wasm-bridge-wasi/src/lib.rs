#[cfg(not(target_arch = "wasm32"))]
pub use wasmtime_wasi::*;

#[cfg(not(target_arch = "wasm32"))]
pub use wasmtime::Table;

#[cfg(target_arch = "wasm32")]
pub mod preview2;
