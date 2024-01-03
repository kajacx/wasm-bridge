use anyhow::{bail, Context};
use js_sys::{Function, Object, Reflect, WebAssembly};
use wasm_bindgen_futures::JsFuture;

use crate::{
    direct::ModuleMemory,
    helpers::{map_js_error, static_str_to_js},
    DropHandles, Engine, Result, ToJsValue,
};

use super::*;

pub struct Component {
    module_core: WebAssembly::Module,
    pub(crate) module_core2: Option<WebAssembly::Module>,
    // core3: Option<WebAssembly::Module>,
}

impl Component {
    pub fn new(_engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
        let files = ComponentLoader::generate_files(bytes.as_ref())?;

        // TODO: maybe we can give the bytes out more effectively? With memory view perhaps?
        let module_core = WebAssembly::Module::new(&files.core.to_js_value())
            .map_err(map_js_error("Synchronously compile main core"))?;

        let module_core2 = if let Some(core2) = files.core2 {
            Some(
                WebAssembly::Module::new(&core2.to_js_value())
                    .map_err(map_js_error("Synchronously compile main core"))?,
            )
        } else {
            None
        };

        Ok(Self {
            module_core,
            module_core2,
        })
    }

    pub async fn new_async(_engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
        let files = ComponentLoader::generate_files(bytes.as_ref())?;

        let promise = WebAssembly::compile(&files.core.to_js_value());
        let module = JsFuture::from(promise)
            .await
            .map_err(map_js_error("Asynchronously compile main core"))?;
        let module_core = module.into();

        let module_core2 = if let Some(core2) = files.core2 {
            let promise = WebAssembly::compile(&core2.to_js_value());
            let module = JsFuture::from(promise)
                .await
                .map_err(map_js_error("Asynchronously compile main core"))?;
            Some(module.into())
        } else {
            None
        };

        Ok(Self {
            module_core,
            module_core2,
        })
    }

    pub(crate) fn instantiate(
        &self,
        imports: &Object,
        drop_handles: DropHandles,
        memory: ModuleMemory,
    ) -> Result<Instance> {
        let instance_core = WebAssembly::Instance::new(&self.module_core, imports)
            .map_err(map_js_error("Synchronously instantiate main core"))?;

        let (instance_memory, realloc) =
            Self::get_memory_and_realloc(&instance_core, "cabi_realloc")?;
        memory.set(instance_memory, realloc);

        Instance::new(instance_core, drop_handles)
    }

    pub(crate) async fn instantiate_async(
        &self,
        imports: &Object,
        drop_handles: DropHandles,
        memory: ModuleMemory,
    ) -> Result<Instance> {
        let promise = WebAssembly::instantiate_module(&self.module_core, imports);
        let instance = JsFuture::from(promise)
            .await
            .map_err(map_js_error("Asynchronously instantiate main core"))?;
        let instance_core = instance.into();

        let (instance_memory, realloc) =
            Self::get_memory_and_realloc(&instance_core, "cabi_realloc")?;
        memory.set(instance_memory, realloc);

        Instance::new(instance_core, drop_handles)
    }

    pub(crate) fn instantiate_wasi(
        &self,
        imports: &Object,
        drop_handles: DropHandles,
        memory: ModuleMemory,
        (wasi_import, dyn_fns, wasi_memory): (&Object, &DynFns, ModuleMemory),
    ) -> Result<Instance> {
        let instance_core = WebAssembly::Instance::new(&self.module_core, imports)
            .map_err(map_js_error("Synchronously instantiate main core"))?;

        Self::prepare_wasi_imports(&instance_core, wasi_imports, &memory)?;

        let wasi_core = WebAssembly::Instance::new(
            self.module_core2.as_ref().context("Get wasi core")?,
            wasi_imports,
        )
        .map_err(map_js_error("Synchronously instantiate wasi core"))?;

        (wasi_memory_, wasi_realloc) =
            Self::get_memory_and_realloc(&wasi_core, "cabi_import_realloc")?;
        wasi_memory.set(wasi_memory_, wasi_realloc);

        Self::link_wasi_exports(&wasi_core, dyn_fns)?;

        Instance::new(instance_core, drop_handles)
    }

    pub(crate) async fn instantiate_wasi_async(
        &self,
        imports: &Object,
        drop_handles: DropHandles,
        memory: ModuleMemory,
        (wasi_import, dyn_fns, wasi_memory): (&Object, &DynFns, ModuleMemory),
    ) -> Result<Instance> {
        let promise = WebAssembly::instantiate_module(&self.module_core, imports);
        let instance = JsFuture::from(promise)
            .await
            .map_err(map_js_error("Asynchronously instantiate main core"))?;
        let instance_core: WebAssembly::Instance = instance.into();

        Self::prepare_wasi_imports(&instance_core, wasi_imports, &memory)?;

        let promise = WebAssembly::instantiate_module(
            self.module_core2.as_ref().context("Get wasi core")?,
            wasi_imports,
        );
        let instance = JsFuture::from(promise)
            .await
            .map_err(map_js_error("Asynchronously instantiate wasi core"))?;
        let wasi_core: WebAssembly::Instance = instance.into();

        (wasi_memory_, wasi_realloc) =
            Self::get_memory_and_realloc(&wasi_core, "cabi_import_realloc")?;
        wasi_memory.set(wasi_memory_, wasi_realloc);

        Self::link_wasi_exports(&wasi_core, dyn_fns)?;

        Instance::new(instance_core, drop_handles)
    }

    fn prepare_wasi_imports(
        instance_core: &WebAssembly::Instance,
        wasi_imports: &Object,
        main_memory: &ModuleMemory,
    ) -> Result<()> {
        let (module_memory, cabi_realloc) = get_memory_and_realloc(instance_core, "cabi_realloc");
        main_memory.set(module_memory, cabi_realloc);

        let main_module_obj = Object::new();
        Reflect::set(
            &main_module_obj,
            static_str_to_js("cabi_realloc"),
            &cabi_realloc,
        )
        .expect("main module is an object");

        let env_obj = Object::new();
        Reflect::set(&env_obj, static_str_to_js("memory"), &module_memory)
            .expect("env is an object");

        Reflect::set(
            &wasi_imports,
            static_str_to_js("__main_module__"),
            &main_module_obj,
        )
        .expect("wasi imports is an object");
        Reflect::set(&wasi_imports, static_str_to_js("env"), &env_obj)
            .expect("wasi imports is an object");

        Ok(())
    }

    fn link_wasi_exports(wasi_core: &WebAssembly::Instance, dyn_fns: &DynFns) -> Result<()> {
        let wasi_exports = wasi_core.exports();

        for (name, dyn_fn) in dyn_fns {
            let exported_fn = Reflect::get(&wasi_exports, &(*name).into())
                .map_err(map_js_error("wasi exports get fn"))?;

            // If the function is missing, we ignore it, only used imports are present
            if exported_fn.is_function() {
                Reflect::set_u32(&dyn_fn, 0, &exported_fn).expect("dyn_fn is an array");
            }
        }

        Ok(())
    }

    fn get_memory_and_realloc(
        instance: WebAssembly::Instance,
        realloc_name: &'static str,
    ) -> Result<(WebAssembly::Memory, Function)> {
        let exports = instance.exports();

        let memory = Reflect::get(&exports, static_str_to_js("memory"))
            .map_err(map_js_error("get memory export"))?;
        if !memory.is_object() {
            bail!("Instance's memory is not an object, it's {memory:?} instead.");
        }

        let realloc = Reflect::get(&exports, static_str_to_js(realloc_name))
            .map_err(map_js_error("get realloc export"))?;
        if !realloc.is_function() {
            bail!(
                "Instance's realloc '{realloc_name}' is not a function, it's {realloc:?} instead."
            );
        }

        Ok((memory.into(), realloc.into()))
    }
}

pub async fn new_component_async(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Component> {
    Component::new_async(engine, bytes).await
}
