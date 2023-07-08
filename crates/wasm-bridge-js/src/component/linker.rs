use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

use crate::{AsContextMut, DataHandle, DropHandler, Engine, Result};

use super::*;

pub struct Linker<T> {
    fns: Vec<PreparedFn<T>>,
}

impl<T> Linker<T> {
    pub fn new(_engine: &Engine) -> Self {
        Self { fns: vec![] }
    }

    pub fn instantiate(
        &self,
        mut store: impl AsContextMut<Data = T>,
        component: &Component,
    ) -> Result<Instance> {
        let import_object: JsValue = js_sys::Object::new().into();
        let mut closures = vec![];

        for function in self.fns.iter() {
            let drop_handler = function
                .add_to_imports(&import_object, store.as_context_mut().data_handle().clone());
            closures.push(drop_handler);
        }

        component.instantiate(store, &import_object, closures)
    }

    pub fn root(&mut self) -> &mut Self {
        self
    }

    pub fn func_wrap<Params, Results, F>(&mut self, name: &str, func: F) -> Result<()>
    where
        T: 'static,
        F: IntoMakeClosure<T, Params, Results>,
    {
        self.fns
            .push(PreparedFn::new(name, func.into_make_closure()));

        Ok(())
    }
}

struct PreparedFn<T> {
    name: String,
    creator: MakeClosure<T>,
}

impl<T> PreparedFn<T> {
    fn new(name: &str, creator: MakeClosure<T>) -> Self {
        Self {
            // name: name.to_lower_camel_case(), // Import name is in kebab-case on purpose
            name: name.into(),
            creator,
        }
    }

    #[must_use]
    fn add_to_imports(&self, imports: &JsValue, handle: DataHandle<T>) -> DropHandler {
        let (js_val, handler) = (self.creator)(handle);

        let object: JsValue = Object::new().into();
        Reflect::set(&object, &"default".into(), &js_val).expect("object is object");

        Reflect::set(imports, &self.name.as_str().into(), &object).expect("imports is object");

        handler
    }
}
