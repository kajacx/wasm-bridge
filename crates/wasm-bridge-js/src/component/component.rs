use js_sys::{Object, WebAssembly};
use wasm_bindgen_futures::JsFuture;

use crate::{direct::ModuleMemory, helpers::map_js_error, DropHandles, Engine, Result, ToJsValue};

use super::*;

pub struct Component {
    module_core: WebAssembly::Module,
    // core2: Option<WebAssembly::Module>,
    // core3: Option<WebAssembly::Module>,
}

impl Component {
    pub fn new(_engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
        let files = ComponentLoader::generate_files(bytes.as_ref())?;

        // TODO: maybe we can give the bytes out more effectively? With memory view perhaps?
        let module_core = WebAssembly::Module::new(&files.core.to_js_value())
            .map_err(map_js_error("Synchronously compile main core"))?;

        Ok(Self { module_core })
    }

    pub async fn new_async(_engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
        let files = ComponentLoader::generate_files(bytes.as_ref())?;

        let promise = WebAssembly::compile(&files.core.to_js_value());
        let module = JsFuture::from(promise)
            .await
            .map_err(map_js_error("Asynchronously compile main core"))?;

        let module_core = module.into();

        Ok(Self { module_core })
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
}

pub async fn new_component_async(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Component> {
    Component::new_async(engine, bytes).await
}
