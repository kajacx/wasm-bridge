#[cfg(not(target_arch = "wasm32"))]
mod sys;

#[cfg(not(target_arch = "wasm32"))]
pub use sys::*;

#[cfg(target_arch = "wasm32")]
pub use wasm_bridge_js::*;
#[cfg(all(target_arch = "wasm32", feature = "wasi"))]
pub use wasm_bridge_wasi::*;
