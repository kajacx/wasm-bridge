use std::{borrow::Borrow, collections::HashMap};

use js_sys::{Function, Object, Reflect, Uint8Array, WebAssembly};
use wasm_bindgen::prelude::*;
use zip::{read::ZipFile, ZipArchive};

use crate::{AsContextMut, Engine, FuncId, Result};

use super::{ExportsRoot, Instance};

pub struct Component {
    instantiate: Function,
    compile_core: JsValue,
    instantiate_core: JsValue,
}

impl Component {
    pub fn new(_engine: &Engine, bytes: &[u8]) -> Result<Self> {
        let cursor = std::io::Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor).unwrap();

        let mut wasm_cores = HashMap::<String, Vec<u8>>::new();
        let mut instantiate = Option::<Function>::None;

        for i in 0..archive.len() {
            let file = archive.by_index(i).unwrap();
            let filename = file.name().to_owned();

            if filename.ends_with(".wasm") {
                let file_bytes = Self::load_wasm_core(file)?;
                wasm_cores.insert(filename, file_bytes); // FIXME: remove folder from filename
            } else if filename.ends_with("sync_component.js") {
                instantiate = Some(Self::load_instantiate(file)?);
            }
        }

        let compile_core = Self::make_compile_core(wasm_cores);
        // panic!("WHAT IS COMPILE CORE? {:?}", compile_core);
        let instantiate_core = Self::make_instantiate_core();

        Ok(Self {
            instantiate: instantiate.unwrap(),
            compile_core,
            instantiate_core,
        })
    }

    pub(crate) fn instantiate(
        &self,
        mut store: impl AsContextMut,
        import_object: &JsValue,
    ) -> Result<Instance> {
        let exports = self.instantiate.call3(
            &JsValue::UNDEFINED,
            &self.compile_core,
            import_object,
            &self.instantiate_core,
        )?;

        let names = Object::get_own_property_names(&exports.clone().into());
        let mut export_fns = HashMap::<String, FuncId>::new();
        let context = store.as_context_mut();

        for i in 0..names.length() {
            let name = Reflect::get_u32(&names, i)?;
            let function: Function = Reflect::get(&exports, &name)?.into();
            export_fns.insert(name.as_string().unwrap(), context.add_function(function));
        }

        Ok(Instance::new(ExportsRoot::new(export_fns)))
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

    fn make_compile_core(wasm_cores: HashMap<String, Vec<u8>>) -> JsValue {
        let closure = Closure::<dyn Fn(String) -> WebAssembly::Module>::new(move |name: String| {
            // let name = format!("out-dir/{name}"); // TODO: bad, bad hack
            let bytes = wasm_cores.get(&name).unwrap();
            let byte_array = Uint8Array::from(bytes.borrow());
            WebAssembly::Module::new(&byte_array.into()).unwrap()
        });

        // FIXME: save the closure so it isn't dropped
        // closure.as_ref().into()
        closure.into_js_value()
    }

    fn make_instantiate_core() -> JsValue {
        let closure = Closure::<dyn Fn(WebAssembly::Module) -> WebAssembly::Instance>::new(
            |module: WebAssembly::Module| {
                let imports = Object::new();
                WebAssembly::Instance::new(&module, &imports).unwrap()
            },
        );

        // FIXME: save the closure so it isn't dropped
        // closure.as_ref().into()
        closure.into_js_value()
    }
}
