pub use wasmtime::*;

#[cfg(feature = "component-model")]
pub mod component {
    pub use wasmtime::component::*;

    pub use wasm_bridge_macros::bindgen_sys as bindgen;
}
