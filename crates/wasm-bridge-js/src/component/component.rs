use std::collections::HashMap;

use anyhow::{bail, Context};
use js_sys::{Array, Object, Reflect, WebAssembly};
use wasm_bindgen::JsValue;
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

        Instance::new(instance_core, drop_handles, memory)
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

        Instance::new(instance_core, drop_handles, memory)
    }

    pub(crate) fn instantiate_wasi(
        &self,
        imports: &Object,
        wasi_imports: &Object,
        dyn_fns: &HashMap<String, Array>,
        drop_handles: DropHandles,
        memory: ModuleMemory,
    ) -> Result<Instance> {
        let instance_core = WebAssembly::Instance::new(&self.module_core, imports)
            .map_err(map_js_error("Synchronously instantiate main core"))?;

        let exports = instance_core.exports();
        let cabi_realloc = Reflect::get(&exports, static_str_to_js("cabi_realloc"))
            .map_err(map_js_error("Get cabi realloc"))?;
        let module_memory = Reflect::get(&exports, static_str_to_js("memory"))
            .map_err(map_js_error("Get core memory"))?;

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

        let wasi_core = WebAssembly::Instance::new(
            self.module_core2.as_ref().context("Get wasi core")?,
            wasi_imports,
        )
        .map_err(map_js_error("Synchronously instantiate wasi core"))?;

        let wasi_exports = wasi_core.exports();

        for (name, dyn_fn) in dyn_fns {
            let exported_fn = Reflect::get(&wasi_exports, &name.into())
                .map_err(map_js_error("wasi exports get fn"))?;
            if !exported_fn.is_function() {
                bail!("Missing exported function {name} in wasi exports");
            }
            Reflect::set_u32(&dyn_fn, 0, &exported_fn).expect("dyn_fn is an array");
        }

        Instance::new(instance_core, drop_handles, memory)
    }

    pub(crate) async fn instantiate_wasi_async(
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

        Instance::new(instance_core, drop_handles, memory)
    }
}

pub async fn new_component_async(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Component> {
    Component::new_async(engine, bytes).await
}
