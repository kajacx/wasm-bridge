use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

use crate::*;

pub struct Linker<T> {
    fns: Vec<PreparedFn<T>>,
}

impl<T> Linker<T> {
    pub fn new(_engine: &Engine) -> Self {
        Self { fns: vec![] }
    }

    pub fn instantiate(
        &self,
        store: impl AsContextMut<Data = T>,
        module: &Module,
    ) -> Result<Instance, Error> {
        let store = store.as_context();

        let imports: JsValue = Object::new().into();
        let mut drop_handles = vec![];

        for func in self.fns.iter() {
            let drop_handle = func.add_to_imports(&imports, store.data_handle());
            drop_handles.push(drop_handle);
        }

        Instance::new_with_imports(module, &imports.into(), drop_handles)
    }

    pub fn func_wrap<Params, Results, F>(
        &mut self,
        module: &str,
        name: &str,
        func: F,
    ) -> Result<&mut Self, Error>
    where
        F: IntoMakeClosure<T, Params, Results> + 'static,
    {
        let creator = func.into_make_closure();

        self.fns.push(PreparedFn::new(module, name, creator));

        Ok(self)
    }
}

#[derive(Debug)]
pub struct DropHandler(Box<dyn std::fmt::Debug>);

impl DropHandler {
    pub(crate) fn new<T: std::fmt::Debug + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }

    pub(crate) fn from_closure(
        closure: impl AsRef<JsValue> + std::fmt::Debug + 'static,
    ) -> (JsValue, Self) {
        let js_value = closure.as_ref().clone();

        (js_value, Self::new(closure))
    }
}

struct PreparedFn<T> {
    module: String,
    name: String,
    creator: MakeClosure<T>,
}

impl<T> PreparedFn<T> {
    fn new(module: &str, name: &str, creator: MakeClosure<T>) -> Self {
        Self {
            module: module.into(),
            name: name.into(),
            creator,
        }
    }

    #[must_use]
    fn add_to_imports(&self, imports: &JsValue, handle: &DataHandle<T>) -> DropHandler {
        let module = Self::module(imports, &self.module);

        let (js_val, handler) = (self.creator)(handle.clone());

        Reflect::set(&module, &self.name.as_str().into(), &js_val).expect("module is object");

        handler
    }

    fn module(imports: &JsValue, module: &str) -> JsValue {
        let module_str: JsValue = module.into();
        let existing = Reflect::get(imports, &module_str).expect("imports is object");

        if existing.is_object() {
            existing
        } else {
            let new_module: JsValue = Object::new().into();
            Reflect::set(imports, &module_str, &new_module).expect("imports is object");
            new_module
        }
    }
}
