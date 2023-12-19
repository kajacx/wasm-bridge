use std::{collections::HashMap, rc::Rc};

use js_sys::{Function, WebAssembly};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::{helpers::map_js_error, AsContextMut, DropHandle, Engine, Result, ToJsValue};

use super::*;

pub struct Component {
    core_module: WebAssembly::Module,
    // core2: Option<WebAssembly::Module>,
    // core3: Option<WebAssembly::Module>,
}

impl Component {
    pub fn new(_engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
        let files = ComponentLoader::generate_files(bytes.as_ref())?;

        // TODO: maybe we can give the bytes out more effectively? With memory view perhaps?
        let core_module = WebAssembly::Module::new(&files.core.to_js_value())
            .map_err(map_js_error("Synchronously compile main core"))?;

        Ok(Self { core_module })
    }

    pub async fn new_async(_engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
        let files = ComponentLoader::generate_files(bytes.as_ref())?;

        let promise = WebAssembly::compile(&files.core.to_js_value());
        let module = JsFuture::from(promise)
            .await
            .map_err(map_js_error("Asynchronously compile main core"))?;

        let core_module = module.into();

        Ok(Self { core_module })
    }

    pub(crate) fn instantiate(
        &self,
        _store: impl AsContextMut,
        import_object: &JsValue,
        closures: Rc<[DropHandle]>,
    ) -> Result<Instance> {
        let exports = self
            .instantiate
            .call3(
                &JsValue::UNDEFINED,
                &self.compile_core,
                import_object,
                &self.instantiate_core,
            )
            .map_err(map_js_error("Call component instantiate"))?;

        Ok(Instance::new(
            ExportsRoot::new(exports, &closures)?,
            closures,
        ))
    }

    fn make_compile_core(wasm_cores: Vec<(String, Vec<u8>)>) -> (JsValue, DropHandle) {
        let mut wasm_modules = HashMap::<String, WebAssembly::Module>::new();
        for (name, bytes) in wasm_cores.into_iter() {
            wasm_modules.insert(
                name,
                WebAssembly::Module::new(&bytes.to_js_value()).expect("TODO: user error"),
            );
        }

        let closure = Closure::<dyn Fn(String) -> WebAssembly::Module>::new(move |name: String| {
            wasm_modules.get(&name).expect("TODO: user error").clone()
        });

        DropHandle::from_closure(closure)
    }

    async fn make_compile_core_async(wasm_cores: Vec<(String, Vec<u8>)>) -> (JsValue, DropHandle) {
        let mut wasm_modules = HashMap::<String, WebAssembly::Module>::new();

        // TODO: wait for all futures at once instead
        for (name, bytes) in wasm_cores.into_iter() {
            let promise = WebAssembly::compile(&bytes.to_js_value());
            let future = JsFuture::from(promise);
            let module = future.await.expect("TODO: user error");
            wasm_modules.insert(name, module.into());
        }

        let closure = Closure::<dyn Fn(String) -> WebAssembly::Module>::new(move |name: String| {
            // TODO: verify that Clone is effective
            wasm_modules.get(&name).expect("TODO: user error").clone()
        });

        DropHandle::from_closure(closure)
    }

    fn make_instantiate_core() -> (JsValue, DropHandle) {
        let closure = Closure::<dyn Fn(WebAssembly::Module, JsValue) -> WebAssembly::Instance>::new(
            |module: WebAssembly::Module, imports: JsValue| {
                // TODO: this should be a user error?
                WebAssembly::Instance::new(&module, &imports.into()).unwrap()
            },
        );

        DropHandle::from_closure(closure)
    }
}

pub async fn new_component_async(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Component> {
    Component::new_async(engine, bytes).await
}
