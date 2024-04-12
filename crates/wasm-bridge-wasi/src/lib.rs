#[cfg(not(target_arch = "wasm32"))]
pub use wasmtime_wasi::*;

#[cfg(target_arch = "wasm32")]
mod js;

#[cfg(target_arch = "wasm32")]
pub use js::*;
