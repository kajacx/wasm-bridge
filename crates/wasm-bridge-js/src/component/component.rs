use std::{borrow::Borrow, collections::HashMap};

use js_sys::{Function, Object, Reflect, Uint8Array, WebAssembly};
use wasm_bindgen::prelude::*;
use zip::{read::ZipFile, ZipArchive};

use crate::{helpers, AsContextMut, DropHandler, Engine, Result};

use super::*;

pub struct Component {
    instantiate: Function,
    compile_core: JsValue,
    instantiate_core: JsValue,
    _drop_handles: [DropHandler; 2],
}

impl Component {
    pub fn new(_engine: &Engine, bytes: &[u8]) -> Result<Self> {
        let cursor = std::io::Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor).unwrap();

        let mut wasm_cores = HashMap::<String, Vec<u8>>::new();
        let mut instantiate = Option::<Function>::None;
        let mut version = Option::<String>::None;

        for i in 0..archive.len() {
            let file = archive.by_index(i).unwrap();
            let filename = file.name().to_owned();

            if filename.ends_with(".wasm") {
                let file_bytes = Self::load_wasm_core(file)?;
                wasm_cores.insert(filename, file_bytes); // TODO: remove folder from filename?
            } else if filename.ends_with("sync_component.js") {
                instantiate = Some(Self::load_instantiate(file)?);
            } else if filename.ends_with("version.txt") {
                version = Some(Self::get_version(file)?);
            }
        }

        let (compile_core, drop0) = Self::make_compile_core(wasm_cores);
        let (instantiate_core, drop1) = Self::make_instantiate_core();

        Self::check_version(version.as_deref());

        Ok(Self {
            instantiate: instantiate.unwrap(), // TODO: add user error
            compile_core,
            instantiate_core,
            _drop_handles: [drop0, drop1],
        })
    }

    pub(crate) fn instantiate(
        &self,
        _store: impl AsContextMut,
        import_object: &JsValue,
        closures: Vec<DropHandler>,
    ) -> Result<Instance> {
        let exports = self.instantiate.call3(
            &JsValue::UNDEFINED,
            &self.compile_core,
            import_object,
            &self.instantiate_core,
        )?;

        let names = Object::get_own_property_names(&exports.clone().into());
        let mut export_fns = HashMap::<String, Func>::new();

        for i in 0..names.length() {
            let name = Reflect::get_u32(&names, i)?;
            let function: Function = Reflect::get(&exports, &name)?.into();
            export_fns.insert(name.as_string().unwrap(), Func::new(function));
        }

        Ok(Instance::new(ExportsRoot::new(export_fns), closures))
    }

    fn load_wasm_core(mut file: ZipFile) -> Result<Vec<u8>> {
        let mut file_bytes = Vec::<u8>::new(); // TODO: reuse same vec to make it more efficient
        std::io::copy(&mut file, &mut file_bytes).unwrap();

        Ok(file_bytes)
    }

    fn load_instantiate(mut file: ZipFile) -> Result<Function> {
        let mut file_bytes = Vec::<u8>::new();
        std::io::copy(&mut file, &mut file_bytes).unwrap();
        let text = String::from_utf8(file_bytes).unwrap(); // TODO: this needs to be user error

        let instantiate: Function = js_sys::eval(&text)?.into();
        Ok(instantiate)
    }

    fn get_version(mut file: ZipFile) -> Result<String> {
        let mut file_bytes = Vec::<u8>::new();
        std::io::copy(&mut file, &mut file_bytes).unwrap();
        Ok(String::from_utf8(file_bytes).unwrap()) // TODO: this needs to be user error
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
