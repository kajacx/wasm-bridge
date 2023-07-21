pub use wasmtime::*;

#[cfg(feature = "component-model")]
pub mod component {
    pub use wasmtime::component::*;

    pub use wasm_bridge_macros::bindgen_sys as bindgen;
    pub use wasm_bridge_macros::flags;
    pub use wasm_bridge_macros::ComponentType;
    pub use wasm_bridge_macros::Lift;
    pub use wasm_bridge_macros::Lower;
}
