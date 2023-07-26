use std::{collections::HashMap, rc::Rc};

use js_sys::{Function, WebAssembly};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::{helpers::map_js_error, AsContextMut, DropHandler, Engine, Result, ToJsValue};

use super::*;

pub struct Component {
    instantiate: Function,
    compile_core: JsValue,
    instantiate_core: JsValue,
    _drop_handles: [DropHandler; 2],
}

impl Component {
    pub fn new(_engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
        let files = ComponentLoader::generate_files(bytes.as_ref())?;

        let (compile_core, drop0) = Self::make_compile_core(files.wasm_cores);
        let (instantiate_core, drop1) = Self::make_instantiate_core();

        Ok(Self {
            instantiate: files.instantiate,
            compile_core,
            instantiate_core,
            _drop_handles: [drop0, drop1],
        })
    }

    pub async fn new_async(_engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
        let files = ComponentLoader::generate_files(bytes.as_ref())?;

        let (compile_core, drop0) = Self::make_compile_core_async(files.wasm_cores).await;
        let (instantiate_core, drop1) = Self::make_instantiate_core();

        Ok(Self {
            instantiate: files.instantiate,
            compile_core,
            instantiate_core,
            _drop_handles: [drop0, drop1],
        })
    }

    pub(crate) fn instantiate(
        &self,
        _store: impl AsContextMut,
        import_object: &JsValue,
        closures: Rc<[DropHandler]>,
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

    fn make_compile_core(wasm_cores: Vec<(String, Vec<u8>)>) -> (JsValue, DropHandler) {
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

        DropHandler::from_closure(closure)
    }

    async fn make_compile_core_async(wasm_cores: Vec<(String, Vec<u8>)>) -> (JsValue, DropHandler) {
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

        DropHandler::from_closure(closure)
    }

    fn make_instantiate_core() -> (JsValue, DropHandler) {
        let closure = Closure::<dyn Fn(WebAssembly::Module, JsValue) -> WebAssembly::Instance>::new(
            |module: WebAssembly::Module, imports: JsValue| {
                // TODO: this should be a user error?
                WebAssembly::Instance::new(&module, &imports.into()).unwrap()
            },
        );

        DropHandler::from_closure(closure)
    }
}

pub async fn component_new_async(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Component> {
    Component::new_async(engine, bytes).await
}
