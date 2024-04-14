use ref_cast::RefCast;

pub use wasmtime::*;

#[repr(transparent)]
#[derive(RefCast)]
pub struct Module(pub(crate) wasmtime::Module);

impl Module {
    #[deprecated(
        since = "0.4.0",
        note = "Compiling a module synchronously can panic, please use `new_safe` instead."
    )]
    pub fn new(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
        Ok(Self(wasmtime::Module::new(engine, bytes)?))
    }

    pub async fn new_safe(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
        // This calls `new` on sys, but `new_async` on js.
        #[allow(deprecated)]
        Self::new(engine, bytes)
    }

    #[cfg(feature = "async")]
    pub async fn new_async(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
        Ok(Self(wasmtime::Module::new_async(engine, bytes).await?))
    }
}

#[repr(transparent)]
#[derive(RefCast)]
pub struct Instance(pub(crate) wasmtime::Instance);

impl Instance {
    #[deprecated(
        since = "0.4.0",
        note = "Instantiating a module synchronously can panic, please use `new_safe` instead."
    )]
    pub fn new(store: impl AsContextMut, module: &Module, imports: &[Extern]) -> Result<Self> {
        Ok(Self(wasmtime::Instance::new(store, &module.0, imports)?))
    }

    pub async fn new_safe(
        store: impl AsContextMut,
        module: &Module,
        imports: &[Extern],
    ) -> Result<Self> {
        // This calls `new` on sys, but `new_async` on js.
        #[allow(deprecated)]
        Self::new(store, module, imports)
    }

    #[cfg(feature = "async")]
    pub async fn new_async(
        store: impl AsContextMut,
        module: &Module,
        imports: &[()],
    ) -> Result<Self> {
        let imports = Object::new();
        Ok(Self(
            wasmtime::Instance::new_async(store, &module.0, imports).await?,
        ))
    }

    pub fn get_memory(&self, store: impl AsContextMut, name: &str) -> Option<Memory> {
        self.0.get_memory(store, name)
    }

    pub fn get_func(&self, store: impl AsContextMut, name: &str) -> Option<Func> {
        self.0.get_func(store, name)
    }

    pub fn get_typed_func<Params, Results>(
        &self,
        store: impl AsContextMut,
        name: &str,
    ) -> Result<TypedFunc<Params, Results>>
    where
        Params: WasmParams,
        Results: WasmResults,
    {
        self.0.get_typed_func(store, name)
    }
}

#[repr(transparent)]
#[derive(RefCast)]
pub struct Linker<T>(pub(crate) wasmtime::Linker<T>);

impl<T> Linker<T> {
    pub fn new(engine: &Engine) -> Self {
        Self(wasmtime::Linker::new(engine))
    }

    #[deprecated(
        since = "0.4.0",
        note = "Instantiating a module synchronously can panic, please use `instantiate_safe` instead."
    )]
    pub fn instantiate(
        &self,
        store: impl AsContextMut<Data = T>,
        module: &Module,
    ) -> Result<Instance, Error> {
        Ok(Instance(self.0.instantiate(store, &module.0)?))
    }

    pub async fn instantiate_safe(
        &self,
        store: impl AsContextMut<Data = T>,
        module: &Module,
    ) -> Result<Instance, Error> {
        // This calls `instantiate` on sys, but `instantiate_async` on js.
        #[allow(deprecated)]
        self.instantiate(store, module)
    }

    #[cfg(feature = "async")]
    pub async fn instantiate_async(
        &self,
        store: impl AsContextMut<Data = T>,
        module: &Module,
    ) -> Result<Instance> {
        Ok(Instance(self.0.instantiate_async(store, module).await?))
    }

    pub fn func_new<F>(
        &mut self,
        module: &str,
        name: &str,
        ty: FuncType,
        func: impl Fn(Caller<'_, T>, &[Val], &mut [Val]) -> Result<()> + Send + Sync + 'static,
    ) -> Result<&mut Self> {
        Ok(Self::ref_cast_mut(self.0.func_new(module, name, ty, func)?))
    }

    pub fn func_wrap<Params, Results>(
        &mut self,
        module: &str,
        name: &str,
        func: impl IntoFunc<T, Params, Results>,
    ) -> Result<&mut Self> {
        Ok(Self::ref_cast_mut(self.0.func_wrap(module, name, func)?))
    }
}

#[cfg(feature = "component-model")]
pub mod component {
    pub use wasmtime::component::*;

    pub use wasm_bridge_macros::bindgen_sys as bindgen;
    pub use wasm_bridge_macros::flags_sys as flags;
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

#[cfg(feature = "async")]
pub use async_trait::async_trait;
