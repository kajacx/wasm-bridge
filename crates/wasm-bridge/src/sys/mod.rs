pub use wasmtime::*;

pub async fn new_module_async(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Module> {
    Module::new(engine, bytes)
}

pub async fn instantiate_async<T>(
    store: impl AsContextMut<Data = T>,
    linker: &Linker<T>,
    module: &Module,
) -> Result<Instance> {
    linker.instantiate(store, module)
}

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
    pub async fn new_component_async(
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
