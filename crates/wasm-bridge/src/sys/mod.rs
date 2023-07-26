pub use wasmtime::*;

#[cfg(feature = "component-model")]
pub mod component {
    pub use wasmtime::component::*;

    pub use wasm_bridge_macros::bindgen_sys as bindgen;
    pub use wasm_bridge_macros::flags;
    pub use wasm_bridge_macros::ComponentType;
    pub use wasm_bridge_macros::Lift;
    pub use wasm_bridge_macros::Lower;

    /// Loads component from bytes "asynchronously".
    ///
    /// This is just `Component::new()` on sys,
    /// but on js, this will compile WASM cores asynchronously,
    /// which is better.
    pub async fn component_new_async(
        engine: &wasmtime::Engine,
        bytes: impl AsRef<[u8]>,
    ) -> wasmtime::Result<Component> {
        Component::new(engine, bytes)
    }
}

#[cfg(feature = "wasi")]
pub mod wasi {
    pub use wasmtime_wasi::*;
}

#[cfg(feature = "async")]
pub use async_trait::async_trait;
