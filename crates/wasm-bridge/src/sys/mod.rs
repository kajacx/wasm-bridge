use ref_cast::RefCast;

pub use wasmtime::*;

#[repr(transparent)]
#[derive(RefCast)]
pub struct Module(pub(crate) wasmtime::Module);

impl Module {
    #[deprecated(
        since = "0.4.0",
        note = "Compiling a module synchronously can panic on the web, please use `new_safe` instead."
    )]
    pub fn new(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
        Ok(Self(wasmtime::Module::new(engine, bytes)?))
    }

    pub async fn new_safe(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
        // This just calls `new` on sys, but uses proper async compilation on the web.
        #[allow(deprecated)]
        Self::new(engine, bytes)
    }
}

#[repr(transparent)]
#[derive(RefCast)]
pub struct Instance(pub(crate) wasmtime::Instance);

impl Instance {
    #[deprecated(
        since = "0.4.0",
        note = "Instantiating a module synchronously can panic on the web, please use `new_safe` instead."
    )]
    pub fn new(store: impl AsContextMut, module: &Module, imports: &[Extern]) -> Result<Self> {
        Ok(Self(wasmtime::Instance::new(store, &module.0, imports)?))
    }

    pub async fn new_safe(
        store: impl AsContextMut,
        module: &Module,
        imports: &[Extern],
    ) -> Result<Self> {
        // This just calls `new` on sys, but uses proper async instantiation on the web.
        #[allow(deprecated)]
        Self::new(store, module, imports)
    }

    #[cfg(feature = "async")]
    pub async fn new_async<T>(
        store: impl AsContextMut<Data = T>,
        module: &Module,
        imports: &[Extern],
    ) -> Result<Self>
    where
        T: Send,
    {
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
        note = "Instantiating a module synchronously can panic on the web, please use `instantiate_safe` instead."
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
        // This just calls `instantiate` on sys, but uses proper async instantiation on the web.
        #[allow(deprecated)]
        self.instantiate(store, module)
    }

    #[cfg(feature = "async")]
    pub async fn instantiate_async(
        &self,
        store: impl AsContextMut<Data = T>,
        module: &Module,
    ) -> Result<Instance>
    where
        T: Send,
    {
        Ok(Instance(self.0.instantiate_async(store, &module.0).await?))
    }

    pub fn func_new(
        &mut self,
        module: &str,
        name: &str,
        ty: FuncType,
        func: impl Fn(Caller<T>, &[Val], &mut [Val]) -> Result<()> + Send + Sync + 'static,
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

    use wasmtime::{AsContextMut, Engine, Result};

    pub struct Component(pub(crate) wasmtime::component::Component);

    impl Component {
        #[deprecated(
            since = "0.4.0",
            note = "Compiling a component synchronously can panic on the web, please use `new_safe` instead."
        )]
        pub fn new(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
            Ok(Self(wasmtime::component::Component::new(engine, bytes)?))
        }

        pub async fn new_safe(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
            // This just calls `new` on sys, but uses proper async compilation on the web.
            #[allow(deprecated)]
            Self::new(engine, bytes)
        }
    }

    pub struct Linker<T>(pub wasmtime::component::Linker<T>);

    impl<T> Linker<T> {
        pub fn new(engine: &Engine) -> Self {
            Self(wasmtime::component::Linker::new(engine))
        }

        pub fn instantiate(
            &self,
            store: impl AsContextMut<Data = T>,
            component: &Component,
        ) -> Result<Instance> {
            self.0.instantiate(store, &component.0)
        }

        #[cfg(feature = "async")]
        pub async fn instantiate_async(
            &self,
            store: impl AsContextMut<Data = T>,
            component: &Component,
        ) -> Result<Instance>
        where
            T: Send,
        {
            self.0.instantiate_async(store, &component.0).await
        }

        pub fn root(&mut self) -> LinkerInstance<T> {
            self.0.root()
        }

        pub fn instance(&mut self, name: &str) -> Result<LinkerInstance<T>> {
            self.0.instance(name)
        }
    }
}

#[cfg(feature = "async")]
pub use async_trait::async_trait;
