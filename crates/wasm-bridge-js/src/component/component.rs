use std::{borrow::Borrow, collections::HashMap, rc::Rc};

use anyhow::Context;
use js_sys::{Function, Uint8Array, WebAssembly};
use wasm_bindgen::prelude::*;
use zip::ZipArchive;

use crate::{
    helpers::{self, map_js_error},
    AsContextMut, DropHandler, Engine, Result,
};

use super::*;

pub struct Component {
    instantiate: Function,
    compile_core: JsValue,
    instantiate_core: JsValue,
    _drop_handles: [DropHandler; 2],
}

impl Component {
    pub fn new(_engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
        Self::from_zip(&bytes).or_else(|_| {
            let loader = ComponentLoader::new().unwrap(); // TODO: bad unwrap
            loader.compile_component(bytes.as_ref())
        })
    }

    pub fn from_zip(zip_bytes: impl AsRef<[u8]>) -> Result<Self> {
        let cursor = std::io::Cursor::new(zip_bytes);
        let mut archive = ZipArchive::new(cursor)?;

        let mut version = Option::<String>::None;

        let mut files = vec![];

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let filename = file.name().to_owned();

            let mut file_bytes = Vec::<u8>::with_capacity(file.size() as usize);
            std::io::copy(&mut file, &mut file_bytes)?;

            if filename.ends_with("version.txt") {
                version = Some(String::from_utf8(file_bytes)?);
            } else {
                files.push((filename, file_bytes));
            }
        }

        Self::check_version(version.as_deref());

        Self::from_files(files)
    }

    pub(crate) fn from_files(files: Vec<(String, Vec<u8>)>) -> Result<Self> {
        let mut wasm_cores = HashMap::<String, Vec<u8>>::new();
        let mut instantiate = Option::<Function>::None;

        for (filename, file_bytes) in files.into_iter() {
            if filename.ends_with(".wasm") {
                wasm_cores.insert(filename, file_bytes); // TODO: remove folder from filename?
            } else if filename.ends_with("sync_component.js") {
                instantiate = Some(Self::load_instantiate(&file_bytes)?);
            }
        }

        let instantiate = instantiate.context("component js file not found in zip")?;

        let (compile_core, drop0) = Self::make_compile_core(wasm_cores);
        let (instantiate_core, drop1) = Self::make_instantiate_core();

        Ok(Self {
            instantiate,
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

    fn load_instantiate(file_bytes: &[u8]) -> Result<Function> {
        let text =
            std::str::from_utf8(file_bytes).context("component js file is not valid utf-8")?;

        let instantiate: Function = js_sys::eval(text)
            .map_err(map_js_error("Eval sync_component.js"))?
            .into();

        Ok(instantiate)
    }

    fn check_version(version: Option<&str>) {
        match version {
            Some(version) => {
                let current_version = env!("CARGO_PKG_VERSION");
                if version != current_version {
                    helpers::warn(&format!(
                        "Version mismatch: {} {version}, {} {current_version}",
                        "component was build with wasm-bridge-cli version",
                        "but wasm-bridge is running version"
                    ));
                }
            }
            None => helpers::warn("Missing version file in component zip."),
        }
    }

    fn make_compile_core(wasm_cores: HashMap<String, Vec<u8>>) -> (JsValue, DropHandler) {
        let closure = Closure::<dyn Fn(String) -> WebAssembly::Module>::new(move |name: String| {
            let bytes = wasm_cores.get(&name).unwrap();
            let byte_array = Uint8Array::from(bytes.borrow());
            WebAssembly::Module::new(&byte_array.into()).unwrap()
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

pub fn new_universal_component(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Component> {
    // Component::from_zip(bytes)
    Component::new(engine, bytes)
}
