use std::{collections::HashMap, rc::Rc};

use crate::{
    helpers::{map_js_error, static_str_to_js},
    *,
};
use anyhow::{bail, Context};
use js_sys::{
    Function, Object, Reflect,
    WebAssembly::{self},
};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

pub struct Instance {
    exports: HashMap<String, JsValue>,
    closures: Rc<Vec<DropHandle>>,
}

impl Instance {
    #[deprecated(
        since = "0.3.0",
        note = "Instantiating a module synchronously can panic, please use `new_instance_async` instead."
    )]
    pub fn new(_store: impl AsContextMut, module: &Module, _imports: &[()]) -> Result<Self> {
        let imports = Object::new();
        Self::new_with_imports(module, &imports, vec![])
    }

    pub async fn new_async(
        _store: impl AsContextMut,
        module: &Module,
        _imports: &[()],
    ) -> Result<Self> {
        let imports = Object::new();
        Self::new_with_imports_async(module, &imports, vec![]).await
    }

    pub(crate) fn new_with_imports(
        module: &Module,
        imports: &Object,
        closures: Vec<DropHandle>,
    ) -> Result<Self> {
        let instance = WebAssembly::Instance::new(&module.module, imports)
            .map_err(map_js_error("Instantiate WebAssembly module"))?;

        Self::from_js_object(instance.into(), closures)
    }

    pub(crate) async fn new_with_imports_async(
        module: &Module,
        imports: &Object,
        closures: Vec<DropHandle>,
    ) -> Result<Self> {
        let promise = WebAssembly::instantiate_module(&module.module, imports);

        let instance = JsFuture::from(promise)
            .await
            .map_err(map_js_error("Instantiate WebAssembly module"))?;

        Self::from_js_object(instance, closures)
    }

    fn from_js_object(instance: JsValue, closures: Vec<DropHandle>) -> Result<Self> {
        let exports = Reflect::get(&instance, static_str_to_js("exports"))
            .map_err(map_js_error("Get instance's exports"))?;

        Ok(Self {
            exports: process_exports(exports)?,
            closures: Rc::new(closures),
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

        Some(Func::new(function, self.closures.clone()))
    }

    pub fn get_typed_func<Params: ToJsValue, Results: FromJsValue>(
        &self,
        _store: impl AsContextMut,
        name: &str,
    ) -> Result<TypedFunc<Params, Results>> {
        let function = self.get_func_inner(name)?;

        if function.length() != Params::number_of_args() {
            bail!(
                "Exported function {name} should have {} arguments, but it has {} instead.",
                Params::number_of_args(),
                function.length(),
            );
        }

        Ok(TypedFunc::new(function, self.closures.clone()))
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

pub async fn new_instance_async(
    store: impl AsContextMut,
    module: &Module,
    imports: &[()],
) -> Result<Instance> {
    Instance::new_async(store, module, imports).await
}
