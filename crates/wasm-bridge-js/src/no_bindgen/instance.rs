use std::{collections::HashMap, rc::Rc};

use crate::{helpers::map_js_error, *};
use anyhow::{bail, Context};
use js_sys::{
    Function, Object, Reflect,
    WebAssembly::{self},
};
use wasm_bindgen::JsValue;

pub struct Instance {
    exports: HashMap<String, JsValue>,
    _closures: Rc<Vec<DropHandler>>, // TODO: use Rc<Vec<..>> or Rc<[..]> ?
}

impl Instance {
    pub fn new(_store: impl AsContextMut, module: &Module, _: impl AsRef<[()]>) -> Result<Self> {
        let imports = Object::new();
        Self::new_with_imports(module, &imports, vec![])
    }

    pub(crate) fn new_with_imports(
        module: &Module,
        imports: &Object,
        closures: Vec<DropHandler>,
    ) -> Result<Self> {
        let instance = WebAssembly::Instance::new(&module.module, imports)
            .map_err(map_js_error("Instantiate WebAssembly module"))?;

        let exports = Reflect::get(instance.as_ref(), &"exports".into())
            .map_err(map_js_error("Get instance's exports"))?;

        Ok(Self {
            exports: process_exports(exports)?,
            _closures: Rc::new(closures),
        })
    }

    pub fn get_memory(&self, _store: impl AsContextMut, name: &str) -> Option<Memory> {
        let memory = self.exports.get(name)?;

        if memory.is_object() {
            Some(Memory::new(memory.clone().into()))
        } else {
            None
        }
    }

    pub fn get_func(&self, _store: impl AsContextMut, name: &str) -> Option<Func> {
        let function = self.get_func_inner(name).ok()?;

        Some(Func::new(function, self._closures.clone()))
    }

    pub fn get_typed_func<Params: ToJsValue, Results: FromJsValue>(
        &self,
        _store: impl AsContextMut,
        name: &str,
    ) -> Result<TypedFunc<Params, Results>, Error> {
        let function = self.get_func_inner(name)?;

        if function.length() != Params::number_of_args() {
            bail!(
                "Exported function {name} should have {} arguments, but it has {} instead.",
                Params::number_of_args(),
                function.length(),
            );
        }

        Ok(TypedFunc::new(function, self._closures.clone()))
    }

    fn get_func_inner(&self, name: &str) -> Result<Function> {
        let function = self
            .exports
            .get(name)
            .context("Exported function '{name}' not found")?;

        if !function.is_function() {
            bail!("Exported object '{function:?}' with name '{name}' is not a function");
        }

        Ok(function.clone().into())
    }
}

fn process_exports(js_exports: JsValue) -> Result<HashMap<String, JsValue>> {
    if !js_exports.is_object() {
        bail!(
            "WebAssembly exports must be an object, got '{:?}' instead",
            js_exports
        );
    }

    // TODO: this is duplicated somewhere, but here ...
    let js_exports: Object = js_exports.into();
    let names = Object::get_own_property_names(&js_exports);
    let len = names.length();

    let mut exports = HashMap::new();
    for i in 0..len {
        let name_js = Reflect::get_u32(&names, i).expect("names is array");
        let name = name_js.as_string().expect("name is string");
        let export = Reflect::get(&js_exports, &name_js).expect("js_exports is object");
        exports.insert(name, export);
    }
    Ok(exports)
}
